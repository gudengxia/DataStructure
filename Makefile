INCLUDE = ./include/
SRC = ./src
CFLAGS = -g
OBJ = mydef.o SqList.o LinkList.o

app = app_list_sq app_list_l

all: ${app}

app_list_sq: app/app_list_sq.c liblist.dll 
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS}

app_list_l: app/app_list_l.c liblist.dll
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS}

liblist.dll:  ${OBJ}
	gcc -fPIC --shared $^ -o $@  
	rm -rf ${OBJ}

${OBJ}: %.o: ${SRC}/%.c
	gcc -o $@ $< -c -I${INCLUDE} ${CFLAGS}

.PHONY:clean
clean:
	rm -f liblist.dll ${app}