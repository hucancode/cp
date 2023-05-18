class FizzBuzz {
private:
    int n;
    mutex fizzMtx,buzzMtx,fbMtx,numberMtx;
public:
    FizzBuzz(int n) {
        this->n = n;
        fizzMtx.lock();
        buzzMtx.lock();
        fbMtx.lock();
    }

    // printFizz() outputs "fizz".
    void fizz(function<void()> printFizz) {
        for(int i=0;i<(n/3 - n/15);i++) {
            fizzMtx.lock();
            printFizz();
            numberMtx.unlock();
        }
    }

    // printBuzz() outputs "buzz".
    void buzz(function<void()> printBuzz) {
        for(int i=0;i<(n/5 - n/15);i++) {
            buzzMtx.lock();
            printBuzz();
            numberMtx.unlock();
        }
    }

    // printFizzBuzz() outputs "fizzbuzz".
	void fizzbuzz(function<void()> printFizzBuzz) {
        for(int i=0;i<n/15;i++) {
            fbMtx.lock();
            printFizzBuzz();
            numberMtx.unlock();
        }
    }

    // printNumber(x) outputs "x", where x is an integer.
    void number(function<void(int)> printNumber) {
        for(int i=1;i<=n;i++) {
            numberMtx.lock();
            if(i%15 == 0) {
                fbMtx.unlock();
                continue;
            } else if(i%5 == 0) {
                buzzMtx.unlock();
                continue;
            } else if(i%3 == 0) {
                fizzMtx.unlock();
                continue;
            }
            printNumber(i);
            numberMtx.unlock();
        }
    }
};