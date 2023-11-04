#include "BinaryTree.hpp"
#include <iostream>

using namespace std;

int main()
{
    BinaryTree T;
    cout<<"PreOrder Traverse:"<<endl;
    T.PreOrderTraverse();

    cout<<"InOrder Traverse:"<<endl;
    T.InOrderTraverse();

    cout<<"PostOrder Traverse:"<<endl;
    T.PostOrderTraverse();
    return 0;
}
