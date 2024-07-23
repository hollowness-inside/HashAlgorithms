use super::Sha256;
use super::Sha512;

use crate::HashBytes;

macro_rules! hash_expect {
    ($hash: ident, $inp: literal, $exp: literal) => {
        let digest = $hash::hash_bytes($inp);
        let hex: String = digest
            .into_iter()
            .map(|byte| format!("{byte:02X}"))
            .collect();
        assert_eq!(hex, $exp);
    };
}

#[test]
fn hash512() {
    hash_expect!(Sha512,
        b"",
        "CF83E1357EEFB8BDF1542850D66D8007D620E4050B5715DC83F4A921D36CE9CE47D0D13C5D85F2B0FF8318D2877EEC2F63B931BD47417A81A538327AF927DA3E"
    );
    hash_expect!(Sha512,
        b"a",
        "1F40FC92DA241694750979EE6CF582F2D5D7D28E18335DE05ABC54D0560E0F5302860C652BF08D560252AA5E74210546F369FBBBCE8C12CFC7957B2652FE9A75"
    );
    hash_expect!(Sha512,
        b"Hello World",
        "2C74FD17EDAFD80E8447B0D46741EE243B7EB74DD2149A0AB1B9246FB30382F27E853D8585719E0E67CBDA0DAA8F51671064615D645AE27ACB15BFB1447F459B"
    );
    hash_expect!(Sha512,
        b"Lorem Ipsum",
        "7FFB69027702D73E3376DE17B1377C29EB61A5510BC6196B5A251DC83EF1B444E98138C0F60727BA0E945A62AF0715AE5BB4A6D7435EF1BD8184C7C7C158F317"
    );
}

#[test]
fn hash256() {
    hash_expect!(
        Sha256,
        b"",
        "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855"
    );
    hash_expect!(
        Sha256,
        b"a",
        "CA978112CA1BBDCAFAC231B39A23DC4DA786EFF8147C4E72B9807785AFEE48BB"
    );
    hash_expect!(
        Sha256,
        b"Hello World",
        "A591A6D40BF420404A011733CFB7B190D62C65BF0BCDA32B57B277D9AD9F146E"
    );
    hash_expect!(
        Sha256,
        b"Lorem Ipsum",
        "030DC1F936C3415AFF3F3357163515190D347A28E758E1F717D17BAE453541C9"
    );
}
