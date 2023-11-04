#include "BinaryTree.hpp"
#include "LinkStack.hpp"
#include <iostream>
using namespace std;

BinaryTree::BinaryTree()
{
    CreateBiTree(&root);
}

BinaryTree::~BinaryTree()
{
    DestroyBiTree(&root);
}

void BinaryTree::PreOrderTraverse()
{
    LinkStack<BTNode*> S;
    BTNode* p;

    if(this->root!=NULL)
    {
        S.Push(root);
        cout<<"visit "<<root->data<<endl;
    }
    p = root;

    while(!S.StackEmpty())
    {
        // if !p, it means the node has visited, and this loop is ignored
        while(p && p->lchild)
        {
            p = p->lchild;
            S.Push(p);
            cout<<"visit "<<p->data<<endl;
        }
        S.Pop(&p);
        if(p->rchild)
        {
            p = p->rchild;
            S.Push(p);
            cout<<"visit "<<p->data<<endl;
        }
        else
        {
            p = NULL;
        }
        //if p has right child, then visit it's right subtree;
        //else, set p to NULL, it will block the inner loop
    }
}


void BinaryTree::InOrderTraverse()
{
    LinkStack<BTNode*> S;
    BTNode* p;

    if(this->root!=NULL)
    {
        S.Push(root);
        //cout<<"visit "<<root->data<<endl;
    }
    p = root;

    while(!S.StackEmpty())
    {
        // if !p, it means the node has visited, and this loop is ignored
        while(p && p->lchild)
        {
            p = p->lchild;
            S.Push(p);
            //cout<<"visit "<<p->data<<endl;
        }
        S.Pop(&p);
        cout<<"visit "<<p->data<<endl;
        if(p->rchild)
        {
            p = p->rchild;
            S.Push(p);
            //cout<<"visit "<<p->data<<endl;
        }
        else
        {
            p = NULL;
        }
        //if p has right child, then visit it's right subtree;
        //else, set p to NULL, it will block the inner loop
    }
}



void BinaryTree::PostOrderTraverse()
{
    SState stackstate;
    LinkStack<SState> S;
    BTNode* p;


    p = root;
    if(this->root!=NULL)
    {
        stackstate.p = root;
        stackstate.flag = false;
        S.Push(stackstate);
        //cout<<"visit "<<root->data<<endl;
    }

    while(!S.StackEmpty())
    {
        // if !p, it means the node has visited, and this loop is ignored
        while(p && p->lchild)
        {
            p = p->lchild;
            stackstate.p = p;
            stackstate.flag = false;
            S.Push(stackstate);
            //cout<<"visit "<<p->data<<endl;
        }
        S.Pop(&stackstate);
        //cout<<"visit "<<p->data<<endl;
        if(stackstate.flag || stackstate.p->rchild == NULL)
        {
            cout<<"visit "<<stackstate.p->data<<endl;
            p = NULL;
            continue;
        }

        stackstate.flag = true;
        S.Push(stackstate);

        stackstate.p = p = stackstate.p->rchild;
        stackstate.flag = false;
        S.Push(stackstate);
    }
}

