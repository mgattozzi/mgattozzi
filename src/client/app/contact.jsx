import React from 'react';

class Contact extends React.Component {

  render () {
    const codeStyle = {
      maxWidth: "522px",
    };
    return(
        <div>
          There are a couple of different ways you can reach out to me:
          <ul>
            <li>Work Email:
              <a href="mailto:michael@elsen.co" target="_top"> michael@elsen.co</a>
            </li>
            <li>Personal Email:
              <a href="mailto:michael@gattozzi.com" target="_top"> mgattozzi@gmail.com</a>
            </li>
            <li>Phone: 617-412-5397</li>
            <li><a href="https://twitter.com/mgattozzi">Twitter</a></li>
            <li><a href="https://www.linkedin.com/in/mgattozzi">LinkedIn</a></li>
          </ul>

          If you need to contact me securely you can use my public PGP key
          to encrypt your message. You can find it and proofs of my identity
          on keybase.io <a href="https://keybase.io/mgattozzi">here</a>.
          <pre style={codeStyle} className="center-block"><code>
              -----BEGIN PGP PUBLIC KEY BLOCK-----<br></br>
              Version: GnuPG v2<br></br>
              <br></br>
              mQINBFeLnN8BEADdB4lxskKBM0OAEZCdmlCKdgF/xdj8PFrJI4oDmDrsMliGq91T<br></br>
              iBp1sRAhFf8DiZm11LdAmAXvlDxwl91nZZ/eiLvw7ALPj++F1uSKYDkFEZCtqAbs<br></br>
              ugFb8Ro9qq99rz0m+bteRXWuahbC4M4nFEGBZFKMRMGkd9VMH8rK5NF7oEXlK36h<br></br>
              3wiEttWHOSqQweYupcsaDQB2FnsJyiU3LOHP7Hk9e07d6WCClF2XA2K4i7ewnFQa<br></br>
              gap1hZplwHtS7ITUoRMXZ6PgNYa5VSkjpbqJTFy9TL6A5irOaWqhGEv/pnBmEdD3<br></br>
              JM3kwhQZLThJfQ9sbo+t+pPZkf5UGr5wFxEj0S3z4R4NTmeB6WpGSaY0TCrLiOS2<br></br>
              GvO0RhUK/hKOE6HyYp+y1ZOl3gNbosB2rFAUdnlDAoBfAHnX219qQTAl+TQCYzyy<br></br>
              6f4z7r2TGCA6JJjeDXk5DzpbPKFfOa+/wpgO6GdS8a7T28NudWdYshOOTUGSMp8h<br></br>
              /TX8Dwc/T6+I6cRgItbvi+2FLbvoPVrc9kRCHQU6HBP/eB9aQqKxO6b1iJk+L4q4<br></br>
              sSoh5WZ8HZm67Gf/fUiEj5ZuvOZzUtkgAl+6sgpZNkNSORvyQhNp25ww/UdmE6vT<br></br>
              GqMO4qp1yNupXRgKb5+eYQc2TGJiuVqfQkK2D0W6VsUa05IBYmsLw/C4nQARAQAB<br></br>
              tEVNaWNoYWVsIEdhdHRvenppIChLZXliYXNlIEdQRyBhbmQgSWRlbnRpdHkgS2V5<br></br>
              KSA8bWdhdHRvenppQGdtYWlsLmNvbT6JAjcEEwEIACEFAleLnN8CGwMFCwkIBwIG<br></br>
              FQgJCgsCBBYCAwECHgECF4AACgkQFuL+izhnL2bMHQ//U6GsC0yy9rnnAmqFqukl<br></br>
              +RKWlHQ0Yh5bsrGDBC3P/FMu4V8ESmjza22RginYQUOx2ncUmwj0KZ5gxliOS/zW<br></br>
              8nfimSVCyBBzFGnvbGPvxiqScSZfeTiKh5Ilipd18NyVgxg6rXG8vo+0J6+ngqiF<br></br>
              qcPSnQfFa7EJ3A0QPF3QPqWL0c5ZCDnfa2LDPabCmJIFBkCgShCybG8CR8npwSY7<br></br>
              Mbwyf/GhlDX7LfK0K9tcUDhcTevsbKKf55X0QVs2NDZa+tYpLY1wi5uOltHPfFdh<br></br>
              Bg+rvoIPt3P4oMUAHkhG7Qmf0vyaHahhYmwd+qX1/Y0N0QlDXWvWaQGi4k1d57kg<br></br>
              xp9LWKTFZTyg6BpXhAfx/ibDVyY6cd41kGeE4oMBhHVB9AQPKSPJrNJho5bEYAFP<br></br>
              TvHVzCU2gltMaovOeeRsEygyWBelD61/HWwh0w2IU2XKukKnkV1z/xAtlOmuAfYm<br></br>
              u7qURVt8S7LGDv7WTIu/Gx+25ilVRab50xi21lrS4g4fbVMCjCz+Sn8BoQLcGdFe<br></br>
              R8R86ayy26mfYcLsn/7BbwYsZFkLHEP3U4QjobN0CYJpnJhG5wSUhWioedawcj61<br></br>
              TWNzkW2gNL7CI8xKQ7kufM4F2M6E3bOmY3HiQfQc+maE7Wrtk8nwbTpGjeBsXBH9<br></br>
              v9cTajBZDBkACMsc23DmFVSJAjEEEwEIACUFAleLnN8JEBbi/os4Zy9mAhsDBQsJ<br></br>
              CAcCBhUICQoLAgQWAgMBAADuQw//RlyG1EZFwiaeV3qsgsaUim0c6BXSlmprorja<br></br>
              vU+mESohMylO2bT076Q5cazaIep3jm0VCZJKlb5oTAtGNLnHbr/vNjNgqNRR23be<br></br>
              dwf1nrOMhQxpn7CCgzz4tmcay+v3qX/9diYjgwmyDV/GvZfX5hlC74eHpvfAdCLE<br></br>
              UB1ij3/Q2waNFVs3YZB3S5pBmfdZx1t4dSCADBgihB4t+OxpUABAqZ89H87U62c8<br></br>
              Gz5PpgrBZtREtgI48eQfqiDBYWrDm/wzHRvd8LG7nxOiXytdtrpY4hI/y/2AT46B<br></br>
              zCa3hGsB1GbAvcsEku4wzFua5G/2pewWpeScNsKM+qDSdYsP5SHWLAyvls7EQqsw<br></br>
              M84fedS+RuWf3yr3JFYBzEuPkqVgpo4Q7SEUaDRnGuPyeyJpp5UOnY9WQRlsEwaz<br></br>
              bJgrgjYs3QXT6/PjnnRZvFt8OJKqyQI/et87YtoRp535SAa0kbOCjxQPX78RJD25<br></br>
              3KYKPlWKloPaWF50Da7pz3O5xONgN0mipPj804AjDmgBwDICROqpTqIWEm/vaSMU<br></br>
              tD9fJsjqBrXvjp2maiv6MIbi2QD7sev6gHx5Az99Yat0qluaxPSysYWtxeZjvziE<br></br>
              VqZHUFcYnMdKurKG/uJw9vDAVIc1q8dKWMXiiweiV6zxFnVEpZI3DtqHnpfa8Du1<br></br>
              KdaFyji5Ag0EV4uc3wEQALx8/D+dtoQ1I5J5flXDSg6HK840fVvm4jnrJfJSNZrs<br></br>
              svd5dgDgyPfpeJ1I2DPHPW7iPbKjbkbiz2GjSsYTZ9pzDzfSbxEQeWpCQuCR3LnZ<br></br>
              BEHJh94S2Fpia64TjjAJ0NgYZsTEugpicviUyzoiVF3c767MsdMrpqP/61rDxdb/<br></br>
              gxP01LSvYni8Ccb5KMrko4fFfzf6D9kfLgaOuesd26uX3QiL9g25uL/ah9iiNe4P<br></br>
              VJ8IE8FxLNFMHBO70nvpAs3cxrxpMGJbMuUPn4xjpvzYczaqbc6IQ7VQZFHuhWzg<br></br>
              dGlQAD+/U9ymDfPl1T5huF7eq9qYRzjfaync5ESZgbvPVf7gFDG0Ee44iJPMZJve<br></br>
              B7PAG7/6T+hpyb7A3WCWCbqKgqPZjUJngIdyn63uo0ZWJvx+qXi/HFV4fLVy07Rr<br></br>
              5XGrCbREPmMyJpYVGWy0j/1ujut3TyM2HqjOLsmiP4Zsf4Mzwv3JLNP7WwbKUpPP<br></br>
              z9BDsKhPqlJOkO7BkczSs3JQ29NFKqwpIm67U10AAXTH51LhSDCmjxEHTkes2dUS<br></br>
              BLUlNpIBSt9vuqyoBBBrbxITdFaQO8ICyET6dvY8f/7bTj4pBOz0pOjGVeGFmW05<br></br>
              vUjdWLU9A0Lkek3QzZXm+NfdxIv2jv4fE/B5OcWvymVS1FnzPUJMWD9r3c4ouLtZ<br></br>
              ABEBAAGJAh8EGAEIAAkFAleLnN8CGwwACgkQFuL+izhnL2YSXQ/+N9SwzZAAJqM2<br></br>
              266+M69ekZxAEHC6tpYczEnDUFOqsGq69djSmU1A21qyarevnRw+Jb7Y/yrvBdu4<br></br>
              mGh1bu9MasHJwvrhwKSLoi4zEOnAbREs9tHw9WHF3+5pDfTlv+uPFgvY6OOAwPQr<br></br>
              l3+I8jgVg47d89VRyFfLtpbS4uhpWkwwigflPrfcs4U8+x6CUEAODV/c5emFOq3a<br></br>
              huIQ/m8de2dh7+L33XfnFkD4e6fIlq+tcqF5LDllxqj7WyC+3FNjALhRdsUT03vu<br></br>
              y0Du9PvnCjHJ2T6Ja9Yh7LryfOJZ8pl4CobjTesDfRplbAnkN7W2x/ykAUH21RVZ<br></br>
              kfRtioS61B2Kt+RhnFFPzgqKpVJr3iWvJEfk3XqWfHcIFm65tZlYAnf4/GA/uUtI<br></br>
              4MiiyYC346vghvqSz/cU6TbFTSphg1D0TItq2VQE5muLyDDWZz+VQ257kyi1Ywo+<br></br>
              DUyxMuHAv93iZg1pFFhacZ5F3+mnPvq0Rc3ShMilCzII7q/rl9xufs/zj4bE85ig<br></br>
              moY1lxUGiq3zb5v2C25bEiB93sNd9Ht9sbSfuKuw6CW5Bs3aO75XIae9zQjjlKL+<br></br>
              vKiEIPoIFWpYzgG3OcnkH34ie9rjMHS6hfKIztJ1KgBtc/c7gnX0rMBXIJlHsHZU<br></br>
              u7kwJjqbVssd4Wb2MApXSAQBUpJ70do==KMn7<br></br>
              -----END PGP PUBLIC KEY BLOCK-----
        </code></pre>
        </div>
    );
  }
}

export default Contact;
