INCLUDE = ./include/
SRC = ./src
CFLAGS = -g
OBJ = mydef.o SqList.o LinkList.o SqStack.o LinkStack.o SqQueue.o LinkQueue.o

#OS = ${shell uname}
ifeq (${OS}, Windows_NT)
	LibList = liblist.dll
else
	LibList = liblist.so
endif


app = app_list_sq app_list_l app_stack_sq app_stack_l app_queue_sq app_queue_l maze_stack maze_queue app_kmp app_tree

all: ${app}

app_list_sq: app/app_list_sq.c ${LibList} 
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS}

app_list_l: app/app_list_l.c ${LibList}
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS}

app_stack_sq: app/app_stack.c ${LibList}
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS}

app_stack_l: app/app_stack.c ${LibList}
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS} -DLinkType

app_queue_sq: app/app_queue.c ${LibList}
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS}

app_queue_l: app/app_queue.c ${LibList}
	gcc $< -o $@ -llist -L. -I${INCLUDE} ${CFLAGS} -DLinkType

maze_stack: app/maze/maze.c ${SRC}/LinkStack.c
	gcc $^ -o $@ -I${INCLUDE} ${CFLAGS} -DMazeSolutionByStack 

maze_queue: app/maze/maze.c ${SRC}/LinkQueue.c ${SRC}/LinkStack.c
	gcc $^ -o $@ -I${INCLUDE} ${CFLAGS} -DMazeSolutionByQueue

app_kmp: app/app_kmp.c ${SRC}/kmp.c
	gcc $^ -o $@ -I${INCLUDE} ${CFLAGS} 

app_tree: app/app_tree.c ${SRC}/BiTree.c
	gcc $^ -o $@ -I${INCLUDE} ${CFLAGS}

${LibList}:  ${OBJ}
	gcc -fPIC --shared $^ -o $@  
	#rm -rf ${OBJ}

${OBJ}: %.o: ${SRC}/%.c
	gcc -o $@ $< -c -I${INCLUDE} ${CFLAGS}

.PHONY:clean
clean:
	rm -rf ${OBJ}
clean_all:
	rm -rf ${OBJ} ${LibList} ${app}
