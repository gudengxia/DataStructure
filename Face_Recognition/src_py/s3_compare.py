import face_recognition


def recognition():
    '''****************BEGIN****************'''
    # 导入图片
    known_image_path = "./../pic/step3/in_swift.jpg"
    known_image_cyz = face_recognition.load_image_file(known_image_path)

    unknown_image_1_path = "./../pic/step3/Taylor-Swift.webp"
    unknown_image_2_path = "./../pic/step3/Juli.png"
    unknown_image_1 = face_recognition.load_image_file(unknown_image_1_path)
    unknown_image_2 = face_recognition.load_image_file(unknown_image_2_path)
    '''**************** END ****************'''

    '''****************BEGIN****************'''
    # 编码获取128维特征向量
    cyz_encoding = face_recognition.face_encodings(known_image_cyz)[0]
    unknown_encoding_1 = face_recognition.face_encodings(unknown_image_1)[0]
    unknown_encoding_2 = face_recognition.face_encodings(unknown_image_2)[0]
    '''**************** END ****************'''

    '''****************BEGIN****************'''
    # 比较特征向量值，识别人脸
    face1_result = face_recognition.compare_faces([cyz_encoding], unknown_encoding_1, tolerance=0.5)
    face2_result = face_recognition.compare_faces([cyz_encoding], unknown_encoding_2, tolerance=0.5)
    '''**************** END ****************'''
    return face1_result, face2_result

r1, r2 = recognition()
print([r1, r2])