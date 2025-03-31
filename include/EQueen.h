#ifndef EQUEEN_H
#define EQUEEN_H

typedef struct Loc
{
    int i;
    int j;
}Loc;


class EQueen
{
    public:
        EQueen(int v);
        virtual ~EQueen();
        void PrintBoard();
        int solve();
    protected:
        bool is_safe(Loc pos);
        bool next_safe_pos_in_same_row(Loc cur, Loc* pos);
        bool next_safe_pos_in_next_row(Loc cur, Loc* pos);
    private:
        bool **board;
        int n;
};

#endif // EQUEEN_H
