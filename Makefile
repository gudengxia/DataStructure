INCLUDE = ./include/
SRC = ./src
CFLAGS = -g
OBJ = mydef.o SqList.o LinkList.o SqStack.o LinkStack.o SqQueue.o LinkQueue.o

app = app_list_sq app_list_l app_stack_sq app_stack_l app_queue_sq app_queue_l maze_stack maze_queue

all: ${app}

app_list_sq: app/app_list_sq.c liblist.dll 
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS}

app_list_l: app/app_list_l.c liblist.dll
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS}

app_stack_sq: app/app_stack.c liblist.dll
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS}

app_stack_l: app/app_stack.c liblist.dll
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS} -DLinkType

app_queue_sq: app/app_queue.c liblist.dll
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS}

app_queue_l: app/app_queue.c liblist.dll
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS} -DLinkType

maze_stack: app/maze/maze.c ${SRC}/LinkStack.c
	gcc $^ -o $@ -I${INCLUDE} ${CFLAGS} -DMazeSolutionByStack 

maze_queue: app/maze/maze.c ${SRC}/LinkQueue.c
	gcc $^ -o $@ -I${INCLUDE} ${CFLAGS} -DMazeSolutionByQueue

liblist.dll:  ${OBJ}
	gcc -fPIC --shared $^ -o $@  
	rm -rf ${OBJ}

${OBJ}: %.o: ${SRC}/%.c
	gcc -o $@ $< -c -I${INCLUDE} ${CFLAGS}

.PHONY:clean
clean:
	rm -rf liblist.dll ${app} ${obj}