const input: &str = "hlfhfzffqnnrlnnvnmmgbgwgttbppcrcnnmdmfdmmgwwrrqnrrscrctcbttvcvtvvhchjhccjgjttmddplplqplqlbqlblrrbrvvprpffpmmzpmpcczjzzbwwfssvrrvggncgncgcwczzswwqqjjflffpwfpwpbwpwpdpbpvvqffcfcjffjllncczfzzmhzzmddgdrgrwwjzzdjjsnjsjfsjsjhhcchlccchqchhzzpnngdgndnpnppsdsggbvgvgpprqrqmmlzmzllvrrcvclcwczcqqcdcfcqqmmzbzdzdjzdjdmjdjzdjjcvjvcjvcvssltstttfbtftrfrlrdllrqqfssslccjdcjdjfdjfjqjnqjnjnrnddtnndtnnztzqztqztqzzpmzmggzrgrwwdqwdwcdwdnnmlmgmtmtstwssbffcnclclnclcjjcjpcpqcpqcpqpmqqfccpcjppnspsnnzggnpntndtdqtthwhnhwnwllzhlzhlzzghzghhlhvhwhjhfjjcnjnvjnjvvqccdmmgddllnmnrrdtdnncggfhgfglfgfmfnnpvvggznnwvnwwfgghrrfwrwzwszzzldzdldhlhblhblhbbbgjgsjggmqqmrrzggrhhwpwdpwdpwplpgpbggtssqffbqfbqbnnsqnqfnngcnnmwnmnbmmmslsjllbtbbpllltzzhgzztllsdllrvvhvjvbbhcbhchmchcctbcttvccgwcwpcchrcrdrdggcrrntrrfllcffbdfflrrrgbgrbbbdqbqjbbgbgrrqwqtwqwhwghwhzwwcswsnwnqqjhjhwhfwhffdfgddgjgsjgjhgglhlwhlhssfqfhhdmdnmnppdcddfzzhmhqqntqnnjvnjvjddcvcgcbgbbpjbjtbjjfgftgffplljfjrrhqqpddlssrvsrvrpppsllsdsqqqzzfttqsqzssjbbrnbnnrbbsrshsrrshrhwhbwbrrsrfrttfqtfqqfddvrvjrjvjsjhjsjdjqdjjlqjjjgcjcmcncfcrcwrwsrsslffzszmsszrsssrnrjjvbvpvcppptbbhhrddbcbggbqbmmsqqwggfpfbblmldmmpmwpwfwjfjsjnjmmpllccjzcjcwwpswshhpthhzchctcbcrrrrmvrvrdvvjmmvgmgwglghllvmllzlzzsvzzrmmhnnsjnnpvpwvpwwmvwwdqqdffhhhmccfgfvggchcctrrmdrrhrhnhnzzgpzzgttnhthvhzzqvvvwpwqpqdppsnnrgnnhphphmhcmcrmrvvqlvqqsccqhchzhwwmvmzmczzgsgdsggthgglrlnrlllbdllhwlwltwwcswsgssbhbsbvsbsbwbhbnncrcllttbrbppjccfpfhhgshschsccmrcmmcrrrzvvrcrggmwgwjwnwjjbffjddjnjgngqgdgnndznndvvfqfgfvvrvqrvvpllnsszbsbdbbdzbdbzbqbzznrznzjzpptcptccvwccfscffrftrrsnsvvswvvhbhzzfbzffncchhcnngzzcpcmmfttsntnjjsccqbcqqmzzgppdhppdtppmffgtgvvlzlpptdtttdppqjqtqctcrrzsswwtnwtnwnqqvbbdgjhvmmzpnhfvsbddzhgdwcnfdstvhhbzlzcfjwhlptbhmbmblprtsdmrdhbbbwpplnzgdnrzjmgzgpqbggnqvwwtntzgfwqrztqtdrsnhpfzswptggnvbszdcrmrhhtlrrfnpqrnpwrbmhlfwmdqqdbqrwbzqjbzwrgmbgrtzrhdclqfgsrtsgfwqrnnqgwsncmpgffggssrqvwjlhpsghbqdtzwmvzzvcmzsjqvprvcqwqjbcqcqrhpwwcsrscgmfdppbgvmnrdfrppblznbstnjzwwgstjvtprjbhtpdfgrhdjnjmnlbfwggzhcngvcwvcfpcwdtdppwjrdzsnjlnrzbfqqshlnzvwsmscgpfwjzhtwgfwgzdhbdwwzbsmfwwbmvrlrpswnjlmfbfzhwvcmgwfzssmmtjlwtrpwpwgnspbgchdncbfcpjsvtzjqtwqwjwgbhrbwvhqbcstsgsnwsjmhrlrvzgqhqfrmnrjdrhdjwcwctpdrzctlvnfzmzwhsnfprlzgzjpqvzchlmvbhffhpfjtvsdbvbdmwgvmqpflhwwndbqthmmwshdtspsrvqdflmmzwbqbqmpfdwjmvpbzdnqzfmhzdgldqjjvgpfcqftvjzwnzmfqdggrwlfzdhjnhmtrjbnllgqpntwmhnwtglnqdwbqdblpwnnrdwzpsqzfwqcmhqhnpsdcwvdldphgnrtqzdbnnzdzfttldrqcztlvlrgpdqzrcthslmtqhfvbzrfgnlrprcpbsctqhspbhnjtzrzhqjzszbzdthttqmbznzssftztwlggmdqqdtfllqjzjtvpgjfhtbwtbmtjplqnbdmsvlnqcwtdbdvfjnzgsmpnhbvvwwfbrgffjqfsccdjdwvbsdhqwfzvcpjzjbdjgrdctjplhwbdhhnbnwstvndnnwtsgbhzbvwdshvmnbwsthlrggtmddvjbfzfrnrdrqfjpslrccctzpjbwpdbhlbzfmwbltcqfngdprvfhgcszdtpnrcpdmllfnlspgrdrpwqmqbmrglvlrsmrfqrtzzgjcvqtqzpmghjrvmdmvvqztrjzbzjwdqsmrwpqnswbzhjbzzhdvmnfdsztzdzrjssgnnfqvbtsqrrmcppjgrmnstrnrlwjvvcczqlcbmwqzdfpssfwdfrvwtstwchdgwtrhhcmppcqlmrqlnwqccfphsdhbsbmtjvpcwwjrmrllbpnrmpbvwgwbftpdpphccwqcblcnvvbbppscmnjqgddllbnbvmmqzdffrrjtqwllzgpqrmnlfqrptzqmdmnrfnjpvqvjbsqrhljslgqqcqqtmtbwjrpphtjgjbqpzmzrzjrfjwcdcnbsjfljclffjplnrrcfbmhphtcjrzlrvvjcznpgpnrdwwqvgnbnzqnlcghhgwvhqbvjzfbdvhrzlqfbtlqhpltfjlfpbnjbphmmpntzqgjmwjtchwmlwvfjmfflqzpqnvvrgnbddzlfpdpdjghfgbsfddjspnfdwvqppncmdgfrnvrpcrflhgjgbwdsbwblfcwbtlrrnjjdhbvmrzgsvjwgfnnhqfbvhprlmwwqgclzlbqbrdspcbhftmdscsmpwrggrmnsvjphjmzmmrlrhnmdhwjlbmjchtvsrcplfspsssjznmzcrqnsjjtwjzvlhshbptqwwvjhjvzrhphphsbphpnzpfbwcdnqrhrrvlrwrztlpqnrcfzrncsvpzqzgslrlrwhvtgjmfncldqmvshlmnlpqbgvnwqfcthgrgllmqrjqmfgznspgltpptglpdcvhtzsprprbldbzhbmjsqzwvjggwhsczltcvgwqhspzpzvljwqjgrgtwswjdswlzjzslrsslvqzncjwhbbjpbdthqpgmhfglggmlrgwdsplgscrwstntvrhjzjjlshtgmnnhvsjwfmcjbpzjcstmnpvtbgrfcfdwjljsrfhdphrdcslwhgvlnwltwchplvfzntfgcnlsvzrvnnczhhqdlwjvqprhmtjdtwmppffmszzzqtfrgnhnzqgqzhrjzgntcszstrfhhtptgvswvzvjcgcntmhzzmdgsmtgzhpfvqfnwmsjdhtfgmmbrrfsdlptchqqzqdqjncmtpznfssrcnmcdnthglmfzsfgltrndqsfmdftmfgchbwmzgrtjvgqtshlltthnnpqnzfrchzhdzrrnpzvfzblrmhwdwjnqdptlbvndmmlhzhvsfdlmlhqrgqqzsdqtpczwcrwcbsftvvphfbwjrvrnrcqbbcsqgnhltwzvllljcvpwjgslbmngcdmpdvjlgcnrzwqjdgrblncpqmrgjmpqjzvdmcmwfnwqlszdgwqdfznhsnpsjrfwrqpqmpvhstmzgqblfmcfvwljbhdfhdmqcvrwnqcstwtzgmng";
const start_size: usize = 14;


fn unique_chars(s: &str) -> bool {
    let v: Vec<char> = s.chars().collect();
    let mut y = v.clone();
    y.sort();
    y.dedup();
    v.len() == y.len()
}

fn main() {
    let size = input.len();
    for i in 0..size {
        let t = &input[i..i+start_size];
        if unique_chars(t) {
            println!("{:?}", t);
            println!("{}", i+start_size);
            break;
        }
    }
}
