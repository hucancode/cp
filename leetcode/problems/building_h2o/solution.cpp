class H2O {
    mutex m[3];
    mutex mi;
    int i;
public:
    H2O() {
        i = 0;
        m[1].lock();
        m[2].lock();
    }

    void hydrogen(function<void()> releaseHydrogen) {
        mi.lock();
        //cout<<"release H, wait for m"<<i<<endl;
        m[i].lock();
        //cout<<"release H, locking m"<<i<<endl;
        // releaseHydrogen() outputs "H". Do not change or remove this line.
        releaseHydrogen();
        i++;
        //cout<<"done release H, unlocking m"<<i<<endl;
        m[i].unlock();
        i%=2;
        mi.unlock();
    }

    void oxygen(function<void()> releaseOxygen) {
        //cout<<"release O, waiting for m2"<<endl;
        m[2].lock();
        //cout<<"release O, locking m2"<<endl;
        // releaseOxygen() outputs "O". Do not change or remove this line.
        releaseOxygen();
        //cout<<"done release O, unlocking m0"<<endl;
        m[0].unlock();
    }
};