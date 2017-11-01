import bpy
import struct
import bmesh


def pack_vert(vertices, vert):
    tv = vertices[vert].co
    return struct.pack('>fff', tv[0], tv[1], tv[2])


def pack_tri_verts(vertices, vert1, vert2, vert3):
    return pack_vert(vertices, vert1) + pack_vert(vertices, vert2) + pack_vert(vertices, vert3)


def pack_tri_texcoords(tc1, tc2, tc3):
    return struct.pack('>ffffff', tc1[0], 1.0 - tc1[1], tc2[0], 1.0 - tc2[1], tc3[0], 1.0 - tc3[1])


def pack_tri_indices(i1, i2, i3):
    return struct.pack('>III', i1, i2, i3)


def write_verts(file, mesh):
    tris = bytes()
    texcoords = bytes()
    indices = bytes()
    num_verts = 0
    num_indices = 0
    
    # FIXME: error handling
    
    uv_data = mesh.tessface_uv_textures.active.data
    
    for face_idx, face in enumerate(mesh.tessfaces):
        if len(face.vertices) != 3:
            print("Quads don't work right, use tris")
            return
        
        face_uv = uv_data[face_idx]
        uv = face_uv.uv1, face_uv.uv2, face_uv.uv3
        
        num_indices += 3
        idx_base = face_idx * 3
        indices += pack_tri_indices(idx_base, idx_base + 1, idx_base + 2)

        num_verts += 3
        tris += pack_tri_verts(mesh.vertices, face.vertices[0], face.vertices[1], face.vertices[2])
        texcoords += pack_tri_texcoords(uv[0], uv[1], uv[2])
        
    file.write(struct.pack('>I', num_indices))
    file.write(indices)
    
    file.write(struct.pack('>I', num_verts))
    file.write(tris)
    
    file.write(struct.pack('>I', num_verts))
    file.write(texcoords)
    
    
def pack_matrix4_row(row):
    return struct.pack('>ffff', row[0], row[1], row[2], row[3])


def pack_matrix4(matrix):
    return pack_matrix4_row(matrix[0]) + pack_matrix4_row(matrix[1]) + pack_matrix4_row(matrix[2]) + pack_matrix4_row(matrix[3])


def write_matrix4(file, matrix):
    file.write(pack_matrix4(matrix.transposed()))


def write(filepath):
    scene = bpy.context.scene

    file = open(filepath, 'wb')
    for obj in bpy.context.selected_objects:
        matrix = obj.matrix_world.copy()
        me = obj.to_mesh(scene, True, "PREVIEW")
        me.calc_tessface()
        
        write_matrix4(file, matrix)
        write_verts(file, me)

        bpy.data.meshes.remove(me)
    
    file.close()
    