import face_recognition
import cv2

'''****************BEGIN****************'''
# 加载已知图片
known_image_trump_path = "./../pic/step4/Trump.webp"
known_image_melania_path = "./../pic/step4/Melania.jpg"
known_image_ivaka_path = "./../pic/step4/Ivaka.jpg"
known_image_eric_path = "././../pic/step4/Eric.jpg"


known_image_trump = face_recognition.load_image_file( known_image_trump_path)
known_image_melania = face_recognition.load_image_file(known_image_melania_path )
known_image_ivaka = face_recognition.load_image_file( known_image_ivaka_path)
known_image_eric = face_recognition.load_image_file(known_image_eric_path )


'''**************** END ****************'''

'''****************BEGIN****************'''
# 对图片进行编码，获取128维特征向量

trump_encoding = face_recognition.face_encodings(known_image_trump )[0]
melania_encoding = face_recognition.face_encodings(known_image_melania)[0]
ivaka_encoding = face_recognition.face_encodings( known_image_ivaka)[0]
eric_encoding = face_recognition.face_encodings( known_image_eric)[0]



'''**************** END ****************'''

'''****************BEGIN****************'''
# 存为数组以便之后识别
known_faces = [trump_encoding,
    melania_encoding,
    ivaka_encoding,
    eric_encoding
]
'''**************** END ****************'''

'''****************BEGIN****************'''
# 加载待识别图片
unknown_image_1_path = "./../pic/step4/unknown_trump.webp"
unknown_image_2_path = "./../pic/step4/unknown_family.jpg"
unknown_image_3_path = "./../pic/step4/unknown_ivaka.jpg"
unknown_image_4_path = "../pic/step4/unknown_eric.jpg"
unknown_image_5_path = "./../pic/step4/unknown_unknown.webp"
unknown_image_1 = face_recognition.load_image_file(unknown_image_1_path)
unknown_image_2 = face_recognition.load_image_file(unknown_image_2_path)
unknown_image_3 = face_recognition.load_image_file(unknown_image_3_path)
unknown_image_4 = face_recognition.load_image_file(unknown_image_4_path)
unknown_image_5 = face_recognition.load_image_file(unknown_image_5_path)
'''**************** END ****************'''

'''****************BEGIN****************'''
# 存为数组以遍历识别
unknown_faces = [unknown_image_1, 
    unknown_image_2, 
    unknown_image_3,
    unknown_image_4,
    unknown_image_5
]
'''**************** END ****************'''

# 初始化一些变量
face_locations = []
face_encodings = []
face_names = []
frame_number = 0

for frame in unknown_faces:
    face_names = []

    '''****************BEGIN****************'''
    # 获取人脸区域位置
    face_locations = face_recognition.face_locations(frame)
    # 对图片进行编码，获取128维特征向量
    face_encodings = face_recognition.face_encodings(frame, face_locations)
    '''**************** END ****************'''

    for face_encoding in face_encodings:

        '''****************BEGIN****************'''
        # 识别图片中人脸是否匹配已知图片
        match = face_recognition.compare_faces(known_faces, face_encoding, tolerance=0.5)
        '''**************** END ****************'''

        '''****************BEGIN****************'''
        name = None
        if  match[0]:
            name = "Trump"
        elif  match[1]:
            name = "Malenia"
        elif  match[2]:
            name = "Ivaka"
        elif match[3]:
            name = 'Eric'
        else:
            name = 'Unknown'
        '''**************** END ****************'''

        face_names.append(name)

    # 结果打上标签
    for (top, right, bottom, left), name in zip(face_locations, face_names):
        if not name:
            continue

        '''****************BEGIN****************'''
        # 绘制脸部区域框
        cv2.rectangle(frame, (left, top), (right, bottom), (0, 0, 255), 2)

        # 在脸部区域下面绘制人名
        cv2.rectangle(frame, (left, bottom - 25),
                      (right, bottom), (0, 0, 255), cv2.FILLED)
        font = cv2.FONT_HERSHEY_DUPLEX
        cv2.putText(frame, name, (left + 6, bottom - 6), font, 0.5, (255, 255, 255), 1)
        '''**************** END ****************'''

        #print(frame[left+6, bottom-6])
        #print(frame[left, bottom])

    print(face_locations)
    print(face_names)
    # 保存图片
    image_rgb = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
    path = './../pic/step4/' + 'out_' + name + str(face_locations[0][0]) + '.jpg'
    cv2.imwrite(path, image_rgb)


