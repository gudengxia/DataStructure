'''****************BEGIN****************'''
import face_recognition
image_path = './../pic/step1/in_trump.jpg'
image = face_recognition.load_image_file(image_path)
face_locations = face_recognition.face_locations(image)
print(face_locations)
'''**************** END ****************'''

import cv2
for face_location in face_locations:
    '''****************BEGIN****************'''
    top, right, bottom, left = face_location
    cv2.rectangle(image, (left, top), (right, bottom), (0, 255, 0), 2)

    '''**************** END ****************'''

# ����ͼƬ
image_rgb = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
cv2.imwrite("./../pic/step1/out_trump.jpg", image_rgb)
