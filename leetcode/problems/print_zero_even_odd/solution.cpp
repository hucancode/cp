class ZeroEvenOdd {
private:
    int n;
    int i;
    mutex zeroMtx;
    mutex numberMtx[2];
public:
    ZeroEvenOdd(int n) {
        this->n = n;
        numberMtx[0].lock();
        numberMtx[1].lock();
    }

    // printNumber(x) outputs "x", where x is an integer.
    void zero(function<void(int)> printNumber) {
        for(int i = 1;i<=n;i++) {
            zeroMtx.lock();
            printNumber(0);
            numberMtx[i%2].unlock();
        }
    }

    void even(function<void(int)> printNumber) {
        for(int i = 2;i<=n;i+=2) {
            numberMtx[0].lock();
            printNumber(i);
            zeroMtx.unlock();
        }
    }

    void odd(function<void(int)> printNumber) {
        for(int i = 1;i<=n;i+=2) {
            numberMtx[1].lock();
            printNumber(i);
            zeroMtx.unlock();
        }
    }
};