import bpy
import struct
import bmesh


def pack_vert(matrix, vertices, vert):
    tv = matrix * vertices[vert].co
    return struct.pack('>fff', tv[0], tv[1], tv[2])


def pack_tri_verts(matrix, vertices, vert1, vert2, vert3):
    return pack_vert(matrix, vertices, vert1) + pack_vert(matrix, vertices, vert2) + pack_vert(matrix, vertices, vert3)


def pack_tri_texcoords(tc1, tc2, tc3):
    return struct.pack('>ffffff', tc1[0], 1.0 - tc1[1], tc2[0], 1.0 - tc2[1], tc3[0], 1.0 - tc3[1])


def write_verts(file, mesh, matrix):
    tris = bytes()
    texcoords = bytes()
    num_verts = 0
    
    # FIXME: error handling
    
    uv_data = mesh.tessface_uv_textures.active.data
    
    for face_idx, face in enumerate(mesh.tessfaces):
        face_uv = uv_data[face_idx]
        uv = face_uv.uv1, face_uv.uv2, face_uv.uv3
        
        if len(face.vertices) == 3:
            num_verts += 3
            tris += pack_tri_verts(matrix, mesh.vertices, face.vertices[0], face.vertices[1], face.vertices[2])
            texcoords += pack_tri_texcoords(uv[0], uv[1], uv[2])
        else:
            print("Quads don't work right, use tris")
            return
    
    file.write(struct.pack('>I', num_verts))
    file.write(tris)
    
    file.write(struct.pack('>I', num_verts))
    file.write(texcoords)


def write(filepath):
    scene = bpy.context.scene

    file = open(filepath, 'wb')
    for obj in bpy.context.selected_objects:
        matrix = obj.matrix_world.copy()
        me = obj.to_mesh(scene, True, "PREVIEW")
        me.calc_tessface()
        
        write_verts(file, me, matrix)

        bpy.data.meshes.remove(me)
    
    file.close()
    