class Doughnut {
  cook() {
    print "Fry until golden brown.";
  }
}

class BostonCream < Doughnut {} 

~ cook() method is inherited from `Doughnut` to  'BostonCream'

BostonCream().cook(); 