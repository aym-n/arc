fn returnFunction() {
  var outside = "outside";

  fn inner() {
    print outside;
  }

  return inner;
}

var fun = returnFunction();
fun();