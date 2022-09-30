(fabric
 (name "Headless Hug")
 (scale 105%)
 (seed :left-right)
 (vulcanize :bowtie)
 (features
  (iterations-per-frame 100)
  (push-over-pull 400%))
 (build
  (branch
   (grow :A
    (scale 95%)
    (twist 0 0 0 0 1 0 0)
    (mark :legs))
   (grow :b
    (scale 95%)
    (twist 0 0 0 0 1 0 0)
    (mark :legs))
   (grow :a
    (scale 90%)
    (branch
    (grow :A 3
      (mark :shoulders))
    (grow :C
      (scale 93%)
      (twist 1 0 0 0 1 0 0)
      (mark :hands))))
   (grow :B
    (scale 90%)
    (branch
    (grow :A 3
      (mark :shoulders))
    (grow :C
      (scale 93%)
      (twist 1 0 0 0 1 0 0)
      (mark :hands))))))
 (shape
  (pull-together :legs 5%)
  (pull-together :hands 7%)
  (pull-together :shoulders 5%))
 (pretense
  (wait 10_000)
  (contract-conflicts)
  (wait 10_000)
  (orient :legs)))