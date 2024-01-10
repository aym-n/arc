class Foo {
  init() {
    print this;
    return 10;
  }
}

var foo = Foo();
print foo.init();