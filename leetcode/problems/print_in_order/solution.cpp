class Foo {
    mutex m[2];
public:
    Foo() {
        m[0].lock();
        m[1].lock();
    }

    void first(function<void()> printFirst) {
        
        // printFirst() outputs "first". Do not change or remove this line.
        printFirst();
        m[0].unlock();
    }

    void second(function<void()> printSecond) {
        m[0].lock();
        // printSecond() outputs "second". Do not change or remove this line.
        printSecond();
        m[0].unlock();
        m[1].unlock();
    }

    void third(function<void()> printThird) {
        m[1].lock();
        // printThird() outputs "third". Do not change or remove this line.
        printThird();
        m[1].unlock();
    }
};