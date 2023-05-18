class FooBar {
private:
    int n;
    mutex fooMtx;
    mutex barMtx;

public:
    FooBar(int n) {
        this->n = n;
        barMtx.lock();
    }

    void foo(function<void()> printFoo) {
        for (int i = 0; i < n; i++) {
            fooMtx.lock();
        	// printFoo() outputs "foo". Do not change or remove this line.
        	printFoo();
            barMtx.unlock();
        }
    }

    void bar(function<void()> printBar) {
        
        for (int i = 0; i < n; i++) {
            barMtx.lock();
        	// printBar() outputs "bar". Do not change or remove this line.
        	printBar();
            fooMtx.unlock();
        }
    }
};