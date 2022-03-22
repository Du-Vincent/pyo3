window.BENCHMARK_DATA = {
  "lastUpdate": 1647956836989,
  "repoUrl": "https://github.com/Du-Vincent/pyo3",
  "entries": {
    "pytest-bench": [
      {
        "commit": {
          "author": {
            "email": "1939362+davidhewitt@users.noreply.github.com",
            "name": "David Hewitt",
            "username": "davidhewitt"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "87c79c031904876ad7b8045019c0fd7780c03307",
          "message": "Merge pull request #2234 from davidhewitt/pyclass-args-refactor\n\npyclass: unify pyclass with its pyo3 arguments",
          "timestamp": "2022-03-22T11:38:05Z",
          "tree_id": "5a01c10cbc7cb7678b455c385ed0f3f3feba56ff",
          "url": "https://github.com/Du-Vincent/pyo3/commit/87c79c031904876ad7b8045019c0fd7780c03307"
        },
        "date": 1647956821306,
        "tool": "pytest",
        "benches": [
          {
            "name": "tests/test_pyclasses.py::test_empty_class_init",
            "value": 6616948.306613356,
            "unit": "iter/sec",
            "range": "stddev: 7.1914831674136506e-9",
            "extra": "mean: 151.12706850124576 nsec\nrounds: 192271"
          },
          {
            "name": "tests/test_pyclasses.py::test_empty_class_init_py",
            "value": 8210787.717239295,
            "unit": "iter/sec",
            "range": "stddev: 3.4292076063801405e-9",
            "extra": "mean: 121.79099429160027 nsec\nrounds: 81968"
          },
          {
            "name": "tests/test_pyfunctions.py::test_none_py",
            "value": 9021810.294341493,
            "unit": "iter/sec",
            "range": "stddev: 3.511135336808057e-9",
            "extra": "mean: 110.84249916304611 nsec\nrounds: 86957"
          },
          {
            "name": "tests/test_pyfunctions.py::test_none_rs",
            "value": 9915035.089260928,
            "unit": "iter/sec",
            "range": "stddev: 3.5218939948652083e-9",
            "extra": "mean: 100.85693000550017 nsec\nrounds: 99010"
          },
          {
            "name": "tests/test_pyfunctions.py::test_simple_py",
            "value": 3450290.126637456,
            "unit": "iter/sec",
            "range": "stddev: 1.3533433802994821e-8",
            "extra": "mean: 289.830699244934 nsec\nrounds: 188680"
          },
          {
            "name": "tests/test_pyfunctions.py::test_simple_rs",
            "value": 2283835.009542249,
            "unit": "iter/sec",
            "range": "stddev: 2.2345603286219493e-8",
            "extra": "mean: 437.8600011912503 nsec\nrounds: 153847"
          },
          {
            "name": "tests/test_pyfunctions.py::test_simple_args_py",
            "value": 2981821.9221320366,
            "unit": "iter/sec",
            "range": "stddev: 1.58048827597322e-8",
            "extra": "mean: 335.36543298503824 nsec\nrounds: 196079"
          },
          {
            "name": "tests/test_pyfunctions.py::test_simple_args_rs",
            "value": 1928483.779916997,
            "unit": "iter/sec",
            "range": "stddev: 2.5472476059017435e-8",
            "extra": "mean: 518.5420849334271 nsec\nrounds: 144907"
          },
          {
            "name": "tests/test_pyfunctions.py::test_simple_kwargs_py",
            "value": 1998816.7653793881,
            "unit": "iter/sec",
            "range": "stddev: 3.664375668315137e-8",
            "extra": "mean: 500.295983764482 nsec\nrounds: 158731"
          },
          {
            "name": "tests/test_pyfunctions.py::test_simple_kwargs_rs",
            "value": 1757590.6026501944,
            "unit": "iter/sec",
            "range": "stddev: 3.790275227644099e-8",
            "extra": "mean: 568.9607116090439 nsec\nrounds: 136987"
          },
          {
            "name": "tests/test_pyfunctions.py::test_simple_args_kwargs_py",
            "value": 2001884.5954328387,
            "unit": "iter/sec",
            "range": "stddev: 1.8215074242303114e-8",
            "extra": "mean: 499.52929468666355 nsec\nrounds: 84034"
          },
          {
            "name": "tests/test_pyfunctions.py::test_simple_args_kwargs_rs",
            "value": 1782202.65491136,
            "unit": "iter/sec",
            "range": "stddev: 2.8622676736794442e-8",
            "extra": "mean: 561.1034173056437 nsec\nrounds: 135136"
          },
          {
            "name": "tests/test_pyfunctions.py::test_args_kwargs_py",
            "value": 2465499.9916215255,
            "unit": "iter/sec",
            "range": "stddev: 2.0809982127241048e-8",
            "extra": "mean: 405.59724331755893 nsec\nrounds: 185186"
          },
          {
            "name": "tests/test_pyfunctions.py::test_args_kwargs_rs",
            "value": 5547103.347141933,
            "unit": "iter/sec",
            "range": "stddev: 8.250939280619156e-9",
            "extra": "mean: 180.27426882457968 nsec\nrounds: 196079"
          }
        ]
      }
    ]
  }
}