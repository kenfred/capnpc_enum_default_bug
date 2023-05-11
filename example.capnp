@0x9eb32e19f86ee174;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("example");

struct Example {
  value @0 :Variants = two;

  enum Variants {
    zero @0;
    one @1;
    two @2;
  }
}
