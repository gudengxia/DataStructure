import face_recognition
'''****************BEGIN****************'''
# ��ȡ����������
image_path = "./../pic/step2/in_swift.jpg"
image = face_recognition.load_image_file(image_path)
face_landmarks_list = face_recognition.face_landmarks(image)
print(face_landmarks_list)
'''**************** END ****************'''

import cv2

# ��������������
for face_landmarks in face_landmarks_list:
    '''****************BEGIN****************'''
    for facial_feature in face_landmarks.keys():
        for pt_pos in face_landmarks[facial_feature]:
                cv2.circle(image, pt_pos, 1, (255, 0, 0), 2)
    '''**************** END ****************'''

# ����ͼƬ
image_rgb = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
cv2.imwrite("./../pic/step2/out_swift.jpg", image_rgb)