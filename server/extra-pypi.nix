{ buildPythonPackage, fetchPypi, fetchFromGitHub, python39Packages }:

rec {
  wtforms-components = buildPythonPackage rec {
    pname = "WTForms-Components";
    version = "0.10.5";
    propagatedBuildInputs = with python39Packages; [
      wtforms
      email_validator
      intervals
      validators
    ];
    doCheck = false;
    src = fetchFromGitHub {
      owner = "kvesteri";
      repo = "wtforms-components";
      rev = "0.10.5";
      sha256 = "sha256-6Yu8dwuPQC+E1Irr8PdaeoR0kTOOSHV9rXXhDkTksgQ=";
    };
  };

  intervals = buildPythonPackage rec {
    pname = "intervals";
    version = "0.9.2";
    propagatedBuildInputs = with python39Packages; [ infinity ];
    checkInputs = with python39Packages; [ pytest ];
    src = fetchPypi {
      inherit pname version;
      sha256 = "sha256-x+5WjFg8qFfA2Rr22Q7l4Oit7z9WRtAHa/uHMFrkMJA=";
    };
  };

  infinity = buildPythonPackage rec {
    pname = "infinity";
    version = "1.5";
    propagatedBuildInputs = with python39Packages; [ ];
    checkInputs = with python39Packages; [ pytest ];
    src = fetchPypi {
      inherit pname version;
      sha256 = "sha256-jap8Fc4hAP3M/eISM34M1c8IWGn1TcJjS2ww1hRh7No=";
    };
  };
}
