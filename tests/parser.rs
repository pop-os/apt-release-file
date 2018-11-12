extern crate apt_release_file;
extern crate chrono;

use apt_release_file::{DistRelease, ReleaseEntry};
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

const RELEASE: &str = include_str!("Release");

#[test]
fn parser() {
    assert_eq!(
        RELEASE.parse::<DistRelease>().unwrap(),
        DistRelease {
            architectures: vec!["i386".into(), "amd64".into(), "all".into()],
            codename: "cosmic".into(),
            components: vec!["main".into()],
            date: DateTime::parse_from_rfc2822("Tue, 06 Nov 2018 14:01:53 +0000")
                .map(|tz| tz.with_timezone(&Utc))
                .unwrap(),
            description: "System76 (cosmic 18.10)".into(),
            label: "System76".into(),
            origin: "system76".into(),
            suite: "cosmic".into(),
            version: "18.10".into(),
            sums: {
                let md5sum = vec![
                    ReleaseEntry {
                        sum: "3439b462cea992ac689c9047ce7a5463".into(),
                        size: 3079744,
                        path: "Contents-all".into(),
                    },
                    ReleaseEntry {
                        sum: "47d7a41a9434e78b7367546b28291d5d".into(),
                        size: 135715,
                        path: "Contents-all.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "374db7421ae4ac1bb637238803e5c770".into(),
                        size: 96700,
                        path: "Contents-all.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "aee2c64573cc4a2e66eb85c23420d4ce".into(),
                        size: 55200657,
                        path: "Contents-amd64".into(),
                    },
                    ReleaseEntry {
                        sum: "42957ffe3a8a4e5e8ecccefc7bcd77b2".into(),
                        size: 2285630,
                        path: "Contents-amd64.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "326b87a728d225449c0c416597017bf2".into(),
                        size: 1680124,
                        path: "Contents-amd64.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "662e2604ba48b607652dd304e375ff7f".into(),
                        size: 947,
                        path: "Contents-i386".into(),
                    },
                    ReleaseEntry {
                        sum: "eea4afc66c66728de3018631a6545136".into(),
                        size: 229,
                        path: "Contents-i386.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "5f6eaf2082378735fc3aa13ad94971d4".into(),
                        size: 276,
                        path: "Contents-i386.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "c9bc25de35afc01b49d560d56eed82cd".into(),
                        size: 25003,
                        path: "main/binary-all/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "afdc54832c1f3c8b18eb2146e2963fab".into(),
                        size: 7647,
                        path: "main/binary-all/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "02323ceff0e2b54cb421ae2d7518dbc5".into(),
                        size: 6760,
                        path: "main/binary-all/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "81a5c3b9d0289bbc830656d216bb9364".into(),
                        size: 105,
                        path: "main/binary-all/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "1cf1fde24e0b75eb9c346c95edb1809e".into(),
                        size: 208961,
                        path: "main/binary-amd64/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "f63c57807e8cc916dd3461abe5c4c041".into(),
                        size: 61367,
                        path: "main/binary-amd64/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "e3f31573c9b7f17599b631b36a520cfa".into(),
                        size: 50360,
                        path: "main/binary-amd64/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "5eeef2cff9ccff507aee6f0c88273245".into(),
                        size: 107,
                        path: "main/binary-amd64/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "552066943ef4551fbd08b6c33e1483a8".into(),
                        size: 1093,
                        path: "main/binary-i386/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "4eb2d88752178dcfd24c9d7366bdfe50".into(),
                        size: 710,
                        path: "main/binary-i386/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "b4da04e2f311efeeaf5a271a58edfd7e".into(),
                        size: 796,
                        path: "main/binary-i386/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "b5b62f2922b8120b044eec3f16675707".into(),
                        size: 106,
                        path: "main/binary-i386/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "652e9d70a76280eb68fbe205673b3d96".into(),
                        size: 30781,
                        path: "main/source/Sources".into(),
                    },
                    ReleaseEntry {
                        sum: "b08870da99b4d602e8d1b7a9c2cc8052".into(),
                        size: 8456,
                        path: "main/source/Sources.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "9f311e64b5336d2648186b9ce4de657c".into(),
                        size: 7400,
                        path: "main/source/Sources.xz".into(),
                    },
                ];

                let sha1 = vec![
                    ReleaseEntry {
                        sum: "05ceb29fc29000498a4a49376cf354a5631c55fe".into(),
                        size: 3079744,
                        path: "Contents-all".into(),
                    },
                    ReleaseEntry {
                        sum: "f2f3d619d8d13122d08254f4e97fca17619bc553".into(),
                        size: 135715,
                        path: "Contents-all.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "d1029a506bcf152778e9431a9c359a3bbd596f05".into(),
                        size: 96700,
                        path: "Contents-all.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "e0112fee86eb8601ffbc90592dd3b87552100703".into(),
                        size: 55200657,
                        path: "Contents-amd64".into(),
                    },
                    ReleaseEntry {
                        sum: "2291c72850cd08d048a28ec43c8ea787a06c592e".into(),
                        size: 2285630,
                        path: "Contents-amd64.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "f4a583d312322c4eb1ffb500f9d123f6d61d599c".into(),
                        size: 1680124,
                        path: "Contents-amd64.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "5a586968f89985ada8a5da1fcaa011bc07d185a9".into(),
                        size: 947,
                        path: "Contents-i386".into(),
                    },
                    ReleaseEntry {
                        sum: "d5d80c26d7dea58396d89b013bc9c0e0f61539ce".into(),
                        size: 229,
                        path: "Contents-i386.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "97a19e7754eaa3f0809e204a302ccf1bc2b700af".into(),
                        size: 276,
                        path: "Contents-i386.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "ba41755f635bd1e4e42d505d19058c29d23ab615".into(),
                        size: 25003,
                        path: "main/binary-all/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "d7d34e7ec6c5dd7557555b0aab3f88a5b359d21f".into(),
                        size: 7647,
                        path: "main/binary-all/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "ceef3cc3db16dca8a5615bcf6fd59e9afcc3d906".into(),
                        size: 6760,
                        path: "main/binary-all/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "80469dc20b336ecb5f6efa4422ee5a5a2e05159d".into(),
                        size: 105,
                        path: "main/binary-all/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "d4aa1c1afd6b0f631c52e4e3b33ef6cae41f617f".into(),
                        size: 208961,
                        path: "main/binary-amd64/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "03ec2c47829c318241d6cb7c0ccc3ec62df77e1e".into(),
                        size: 61367,
                        path: "main/binary-amd64/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "aa6c417dff9064552d1720043d5b70454235a2f5".into(),
                        size: 50360,
                        path: "main/binary-amd64/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "9992f01f51cc0ca2aaf5d4e77de60d7a53be2af4".into(),
                        size: 107,
                        path: "main/binary-amd64/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "a264e382afd17549c80bc9d10300a4d53dbb0084".into(),
                        size: 1093,
                        path: "main/binary-i386/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "3224b8546e73fab8ef100c374928bc4d32d121e5".into(),
                        size: 710,
                        path: "main/binary-i386/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "dfd3ea9397f54d2e489c4fc3e8481b55d6927361".into(),
                        size: 796,
                        path: "main/binary-i386/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "402572ee6c083d15386b107ab1e91e74e9a94c61".into(),
                        size: 106,
                        path: "main/binary-i386/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "62cfdfac1cb7bc744e1f3ad56d24edeca7aae0f5".into(),
                        size: 30781,
                        path: "main/source/Sources".into(),
                    },
                    ReleaseEntry {
                        sum: "efd2cb7c0af8979ede7c878b63128fde1fe010ab".into(),
                        size: 8456,
                        path: "main/source/Sources.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "ba4bb9f05c0ef9cf8b204cb938643f9bb132d1ae".into(),
                        size: 7400,
                        path: "main/source/Sources.xz".into(),
                    },
                ];

                let sha256 = vec![
                    ReleaseEntry {
                        sum: "ad740b679291e333d35106735870cf7217d6c76801b45a01a8e7fde58c93aaf0".into(),
                        size: 3079744,
                        path: "Contents-all".into(),
                    },
                    ReleaseEntry {
                        sum: "092b3ac9ed71b8bbb4ed6e93a7078d6bf42de6d249980efa03f9d100ad0c69a6".into(),
                        size: 135715,
                        path: "Contents-all.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "37adf427913772253d743022f46eb8f244afd714ebf8745a874cd1c3851cefe8".into(),
                        size: 96700,
                        path: "Contents-all.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "8a0dcc21bf7e6bbb6d5f5074391284f61905b54580d0af2d777fd92583e115cd".into(),
                        size: 55200657,
                        path: "Contents-amd64".into(),
                    },
                    ReleaseEntry {
                        sum: "6542982f51d85e7bc9f6655d29ede352b0cd34468d6e0748763b14aaf43f2957".into(),
                        size: 2285630,
                        path: "Contents-amd64.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "26ca879ce1d5187c5513bf43e6783fc1d781b96e534e6346fa95b95915830d3d".into(),
                        size: 1680124,
                        path: "Contents-amd64.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "67eb11e9bed8ac046572268f6748bf9dcf7848d771dd3343fba1490a7bbefb8a".into(),
                        size: 947,
                        path: "Contents-i386".into(),
                    },
                    ReleaseEntry {
                        sum: "379f7a6c108bd9feb63848110a556fffb15975cdb22e4eeb25844292cd4c9285".into(),
                        size: 229,
                        path: "Contents-i386.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "40e605dcffb56f5f7ee5950e4a06d2781efdedf29100e4f86ba647074ae5c807".into(),
                        size: 276,
                        path: "Contents-i386.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "d372739361ba48418a5ad14ee060bfb2d45647a4b78a254b080bed890a9b7ded".into(),
                        size: 25003,
                        path: "main/binary-all/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "4b1c502c989d2475fa3897107bb2a344f473a1fde0759d2692fee8bc3a487489".into(),
                        size: 7647,
                        path: "main/binary-all/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "b5d5c234c37007e9be54ae31784f294ce5802e7fffc6e89e9d7965b814dbb267".into(),
                        size: 6760,
                        path: "main/binary-all/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "ec579a836b8abe6d9510e24155ba142571b82e633e5025398a4a38964705372d".into(),
                        size: 105,
                        path: "main/binary-all/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "6d7d7545f6ba8bccbb2da3e9f22e7ff9de9ec440a04a3f30bd2b2712bec2d11d".into(),
                        size: 208961,
                        path: "main/binary-amd64/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "acacf680011d0b6e1663627469c5f18de8faf4d8c1cd813d53850a588602926e".into(),
                        size: 61367,
                        path: "main/binary-amd64/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "b3d65fcbaaab6bcda2d538b1729ce3686510f64ad867d35f89931f175ce399a5".into(),
                        size: 50360,
                        path: "main/binary-amd64/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "70d3beeb2f218f78254a901a2a8780e0dc4e99819a03753ed3591d4f257e3809".into(),
                        size: 107,
                        path: "main/binary-amd64/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "3e8c18948ed4a9e263bc15094fe25b611a4d15e79c4c8f8e2ca7fa4e07f81cfc".into(),
                        size: 1093,
                        path: "main/binary-i386/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "da210483632d115b5c90111ba103adbc85d20a9abbb2225e384298e51918589c".into(),
                        size: 710,
                        path: "main/binary-i386/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "d2ff4e017ed027532c5783873e662843a53a0630a964554993586cf8ee56c942".into(),
                        size: 796,
                        path: "main/binary-i386/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "2fb8a0f30db882e20fcd20a13a2dc8b7318cdc40d4128f88595b36854eec44b7".into(),
                        size: 106,
                        path: "main/binary-i386/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "9504a4ce13ec880d40a3361d65874b12a26f1727f73089cb68fc5a8644959054".into(),
                        size: 30781,
                        path: "main/source/Sources".into(),
                    },
                    ReleaseEntry {
                        sum: "adab95a7bc930ba478d791ea65d2e7615041bd02efc8136a53380faf529a8447".into(),
                        size: 8456,
                        path: "main/source/Sources.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "f74eb83ee5a50b32674100a2f8df4ec1fc0a9f8eea866f9895dd31b5c7438f76".into(),
                        size: 7400,
                        path: "main/source/Sources.xz".into(),
                    },
                ];

                let sha512 = vec![
                    ReleaseEntry {
                        sum: "e9195e6766cf0274a8b865b47aea8995fb62e1878f9df04fb3085d7f7a0b89ad6a9436099e70988436c7af420746413cbe9a0c81353eccfcf76ac54cec97e6b2".into(),
                        size: 3079744,
                        path: "Contents-all".into(),
                    },
                    ReleaseEntry {
                        sum: "1eb509067de8f390c6b4264d480c5f3ea900499959efdc4d75d8d105d670da1a9b99fd5c7516cdf006e35601a70ebdd9f97ab59ad864800dffe107864e1a004a".into(),
                        size: 135715,
                        path: "Contents-all.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "833d3441d7df044acee0a1f8a7e7193333c93aec2becd531d9524dd1c82c0f540bfd482a4f2f378f903e28ca745c35c02618e84ae98ce73845af6b91a5c2fcb8".into(),
                        size: 96700,
                        path: "Contents-all.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "aaa5b6898715f065e70316e7c0a30ef243635131701f0e00a552ea526bf47860f1a39ecd9b7b3ed116ea98e5c6c7f27d533d08edc3e5710cc7a44b49725b67f0".into(),
                        size: 55200657,
                        path: "Contents-amd64".into(),
                    },
                    ReleaseEntry {
                        sum: "e1c2de999e91710af8c32eeba8f1c9419ccd2b5f39dc6fb2d9d91935af9fc6349cd60f1a98442a5fc27a96a22dff8c679e20ffb005fae1972e3a212c8ad295f3".into(),
                        size: 2285630,
                        path: "Contents-amd64.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "d906d940152e88efea321bc5bb68d74ca4178204686056b82dd35e0c2f867a4458393b34a30ff57d591c9e7b42f5b707b4eeca10c3209072bca0c595adc1e8c4".into(),
                        size: 1680124,
                        path: "Contents-amd64.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "4c98e9948b93aa72a527ff8fd798103266e3c802dd53188820fbe45a15b3993cd7bf3dc016687bbbeb9227844d102eb0656254350456c8d816e3ad1eda23ebb1".into(),
                        size: 947,
                        path: "Contents-i386".into(),
                    },
                    ReleaseEntry {
                        sum: "112c6477611d6c76273e316fa596a106a7ce666d83587a8a4bd7b0a5b8258d06a7adbde650766a963333848bd55cc9ffd7fdfe79a1eec2ed0b6e22074b243e25".into(),
                        size: 229,
                        path: "Contents-i386.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "10444d60fef6b1a141c66478245246dfbae5f503042ddb3c6f9430096ebff7a78b52606c21fea1a5a48c17b8c861bf0751d0b7bc93c9963f5bdc431a8ddd6fbf".into(),
                        size: 276,
                        path: "Contents-i386.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "396c4567a9511bfc9301f7f18944a363022233dead6ff325f5421accaca041b21d82371d97b674b56fbb151ccb12053bb36ea1d753bc675b40f9ec0ed00b191f".into(),
                        size: 25003,
                        path: "main/binary-all/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "d9c1e52f7a30b764b28a01a92e8b1d7407db5e5f2bc49f4ddfd2fdf3bcb0f98f2e92397df7ce78f982f67de76272ad7d038e9371bc107c2f37e9642419067db6".into(),
                        size: 7647,
                        path: "main/binary-all/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "c150c21ca4aef9d253d651781f68dcdf5fa205894e6b9246dff180da02531dfd5e895c3edf30cfc678d4a62cc178cf0759e5f165ed84b27eb618ca020522f082".into(),
                        size: 6760,
                        path: "main/binary-all/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "d140448c2b801db11f41eb22029f4cabef47e36bdb5ba329bbc976de06e06b0ad60208bda08485cb3531bb8a68691bd8db2af688106ec16c9e96184c4bcda635".into(),
                        size: 105,
                        path: "main/binary-all/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "8daf22ea89879d5899e391a63aba8bc7ba04aff713731242c243c98889b5cf5fe9f1a670ac71b74bfe7fc45369b973eac00c79bbc88733aca1808d5ed1b19218".into(),
                        size: 208961,
                        path: "main/binary-amd64/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "ccd726aae84f92066cf5677686f998cc3b5cd971f89f0fb656e857017c277bea9df5103bb0f2b5e379a0f18580c185940ebe38dde8515422e84f03f3fc92cd9c".into(),
                        size: 61367,
                        path: "main/binary-amd64/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "ac6452b5406c81ad3fc08d1c7921f22bf220e1ed4176292ec1ad0ab57fb606317c4af8d455f41363578c0f8f8d808abf392a2cbf904c9ac71d6a3266d7a4f627".into(),
                        size: 50360,
                        path: "main/binary-amd64/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "a8c7c95efa29ab08f1246956a891b04c8bc4a16b9ec859a26e6586b766bfe1987187a25bf688d285c67ca58f98b61f62845e6ed7d5e78f25faea4741dd308c73".into(),
                        size: 107,
                        path: "main/binary-amd64/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "12159693832b131a7d02fb44ac527635a92e8e8c9cc1ac9f6a4d60220fca4af2115ab8f298f634ca5d4f9ebd899bd556ef2503fdc800ada5d91839cc1531411a".into(),
                        size: 1093,
                        path: "main/binary-i386/Packages".into(),
                    },
                    ReleaseEntry {
                        sum: "3d6e9727ccbf793e33b686501d0464cd7dcf227141dcd8daad521b9063940d096371dd890f2a50a671d725aa9668bcdf5c4ad9a0bbd1dc64a73169640dc29f80".into(),
                        size: 710,
                        path: "main/binary-i386/Packages.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "f1307b7467689e42a44614a2ee2c78e130f670f901bfadbaa4452f6e9bcf7823529d550686c2a09dc4835ac73fc38a4a9032a1f2dc6345c7555c66d2787434ef".into(),
                        size: 796,
                        path: "main/binary-i386/Packages.xz".into(),
                    },
                    ReleaseEntry {
                        sum: "8024b1b406b2616b7507eca7d9e1cae657a522a03de9a6ae59012131f6bf4809924ae3888f8714615732928ffb47306d396daf18f7c27572324826c26f53eb79".into(),
                        size: 106,
                        path: "main/binary-i386/Release".into(),
                    },
                    ReleaseEntry {
                        sum: "037a60a612ec06bec80f4beb469add9433f10dd5b5bab798c8943517e1469e1ccd8436d42912a1cd37deeeef2757af6ff5420fedf6ffbff0518d4c794271500c".into(),
                        size: 30781,
                        path: "main/source/Sources".into(),
                    },
                    ReleaseEntry {
                        sum: "c42d6e07ade3dd5400171a5990a6aa161c825d1cdf139dc122d3291844abcde46a718044c552f25960799f894b94508bd335cabb09f877fe898167437627337c".into(),
                        size: 8456,
                        path: "main/source/Sources.gz".into(),
                    },
                    ReleaseEntry {
                        sum: "8231f57747d621ecb91d963ea7551058d02256e79b7c9cf278ef07cc539cf641cad8af16e8505cc05242663061c9a2260e553bc441b35f8e9ee88f40886c08ed".into(),
                        size: 7400,
                        path: "main/source/Sources.xz".into(),
                    }
                ];

                let mut map = BTreeMap::new();
                map.insert("MD5Sum".into(), md5sum);
                map.insert("SHA1".into(), sha1);
                map.insert("SHA256".into(), sha256);
                map.insert("SHA512".into(), sha512);
                map
            }
        }
    )
}
