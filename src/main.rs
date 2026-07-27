// LumiClock - Rust + eframe/egui
//
// Cargo.toml dependencies:
// [dependencies]
// eframe = "0.29"
// chrono = "0.4"
// rand = "0.8"
//
// 실행:
// cargo run --release

use chrono::{Local, Timelike, Datelike}; //Datelike 추
use eframe::egui;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng}; //Rng 추가
use std::time::{Duration, Instant};

// ─────────────────────────────────────────────
// 시간대 구분
// ─────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq)]

enum TimeSlot {
    Morning,   // 06~12시
    Afternoon, // 12~18시
    Evening,   // 18~24시
    Night,     // 00~06시
}

impl TimeSlot {
    fn from_hour(h: u32) -> Self {
        match h {
            6..=11 => Self::Morning,
            12..=17 => Self::Afternoon,
            18..=23 => Self::Evening,
            _ => Self::Night,
        }
    }
}

// 요일 한국어 변환 함수
fn korean_weekday(weekday: chrono::Weekday) -> &'static str {
    match weekday {
        chrono::Weekday::Mon => "월요일",
        chrono::Weekday::Tue => "화요일",
        chrono::Weekday::Wed => "수요일",
        chrono::Weekday::Thu => "목요일",
        chrono::Weekday::Fri => "금요일", 
        chrono::Weekday::Sat => "토요일",
        chrono::Weekday::Sun => "일요일",
    }
}

fn get_mock_weather() -> String {
    let conditions = vec![
        "맑음", "흐림", "비", "눈", "안개", "소나기", "천둥번개", "눈/비", "바람", "폭염",
    ];
    let mut rng = thread_rng();
    let condition = conditions.choose(&mut rng).unwrap();
    let temperature: i32 = rng.gen_range(-10..=35);
    format!("{}°C, {}", temperature, condition)
}


fn avatars_morning() -> Vec<&'static str> {
    vec![
        "(=^･ω･^=)",
        "(✿◠‿◠)",
        "(ﾉ´ヮ`)ﾉ*:･ﾟ✧",
        "(*´▽`*)",
        "(＾▽＾)",
        "ヽ(・∀・)ﾉ",
        "(◕‿◕✿)",
        "☆*:.｡.o(≧▽≦)o.｡.:*☆",
        "(｡•̀ᴗ-)✧",
        "(*ゝω・)ﾉ",
    ]
}

fn avatars_afternoon() -> Vec<&'static str> {
    vec![
        "( •̀ ω •́ )✧",
        "(ง •̀_•́)ง",
        "(`・ω・´)",
        "(๑•̀ㅂ•́)و✧",
        "ᕙ(⇀‸↼‶)ᕗ",
        "(￣ー￣)ｂ",
        "(*`・v・)",
        "٩(ˊᗜˋ*)و",
        "(•̀o•́)ง",
        "ヽ(°〇°)ﾉ",
    ]
}

fn avatars_evening() -> Vec<&'static str> {
    vec![
        "(￣▽￣)",
        "(´▽`ʃ♡ƪ)",
        "( ˘ω˘ )zzZ",
        "(｡◕‿◕｡)",
        "(*´ω`*)",
        "( ´ ▽ ` )ﾉ",
        "(ᵔᴥᵔ)",
        "( ˙꒳˙ )",
        "(＠＾◡＾)",
        "╰(*´︶`*)╯",
    ]
}

fn avatars_night() -> Vec<&'static str> {
    vec![
        "(；￣Д￣)",
        "(￣o￣) zzZZzzZZ",
        "( ˘ω˘ ) zzz",
        "(´-ω-`)",
        "(-_-)zzz",
        "(。-ω-)zzz",
        "( ¯ ³¯)♡",
        "(´q｀●)",
        "( ˇ෴ˇ )",
        "(ᴗ_ᴗ)",
    ]
}

// ─────────────────────────────────────────────
// 문구 데이터
// ─────────────────────────────────────────────

fn phrases_morning() -> Vec<&'static str> {
    vec![
        "오늘 하루도 밝게 시작해요.",
        "아침 햇살만큼 따뜻한 하루 되세요.",
        "좋은 아침이에요! 오늘도 잘 부탁해요.",
        "일어나셨군요, 수고하셨어요.",
        "새로운 하루가 기다리고 있어요.",
        "오늘은 어제보다 조금 더 나아질 거예요.",
        "커피 한 잔과 함께 시작해볼까요?",
        "천천히 시작해도 괜찮아요.",
        "아침 공기가 상쾌하지 않나요?",
        "오늘 할 일을 하나씩 적어보아요.",
        "작은 것부터 시작하는 하루예요.",
        "오늘 하루도 건강하게 보내요.",
        "기지개 한 번 크게 켜봐요.",
        "물 한 잔으로 하루를 열어요.",
        "오늘도 당신을 응원해요.",
        "모든 위대한 하루는 아침에 시작돼요.",
        "오늘 하루도 차분하게 나아가요.",
        "아침을 먹고 시작하면 더 힘차요.",
        "오늘은 무엇을 이루어볼까요?",
        "느릿느릿 시작해도 괜찮아요.",
        "창문을 열고 신선한 공기를 마셔요.",
        "잘 잤나요? 오늘도 함께해요.",
        "오늘 하루도 소중히 써봐요.",
        "작은 성취도 오늘 하루를 빛내줘요.",
        "마음을 가다듬고 시작해봐요.",
        "오늘 하루의 첫 단추를 잘 채워봐요.",
        "천리 길도 한 걸음부터예요.",
        "오늘은 새로운 도전을 해볼까요?",
        "기분 좋은 하루가 되기를 바라요.",
        "아침 루틴을 지키면 하루가 달라져요.",
        "오늘 하루도 최선을 다해봐요.",
        "새 아침, 새 시작이에요.",
        "당신의 하루를 응원해요.",
        "오늘도 한 걸음씩 나아가요.",
        "오늘 하루가 당신에게 선물이 되기를.",
        "밝은 미소로 시작해봐요.",
        "오늘도 좋은 일이 생길 거예요.",
        "계획을 세우면 하루가 탄탄해져요.",
        "작은 것에 감사하는 하루 되세요.",
        "오늘 하루도 차분하게 시작해요.",
        "상쾌한 아침이에요, 힘내봐요!",
        "오늘은 어제의 나보다 한 발짝 더.",
        "무엇이든 시작이 반이에요.",
        "오늘도 꾸준히 나아가는 하루예요.",
        "좋은 아침은 좋은 하루를 만들어요.",
        "당신이 깨어있다는 것만으로도 충분해요.",
        "오늘 하루도 조용히 빛나봐요.",
        "오늘 한 가지만 잘해도 성공이에요.",
        "아침의 고요함을 즐겨봐요.",
        "오늘도 함께여서 다행이에요.",
    ]
}

fn phrases_afternoon() -> Vec<&'static str> {
    vec![
        "점심은 맛있게 드셨나요?",
        "오후도 힘차게 달려봐요.",
        "잠깐 스트레칭 한 번 어떤가요?",
        "집중이 잘 되는 시간이에요.",
        "오늘 오후도 잘 하고 있어요.",
        "물 마시는 것 잊지 마세요.",
        "조금만 더 힘내면 돼요.",
        "지금 이 순간에 집중해봐요.",
        "오후의 햇살이 포근하지 않나요?",
        "오늘 하루도 잘 달리고 있어요.",
        "잠깐 눈을 감고 쉬어봐요.",
        "오후 한 잔의 따뜻한 차 어때요?",
        "잘 하고 있어요, 계속 나아가요.",
        "이미 많이 왔어요, 조금만 더.",
        "오후에도 당신을 응원해요.",
        "지금 하는 일이 쌓여서 빛이 돼요.",
        "흐름을 유지하는 것만으로도 충분해요.",
        "오늘 오후도 한 걸음씩 가요.",
        "조급해하지 않아도 돼요.",
        "천천히 해도 괜찮아, 멈추지만 마요.",
        "지금 이 노력이 나중의 나를 만들어요.",
        "잠깐 환기하고 다시 시작해봐요.",
        "오늘 오후의 집중력을 믿어봐요.",
        "피곤하면 잠깐 쉬는 것도 전략이에요.",
        "지금 잘 하고 있어요.",
        "오후의 끝이 보이기 시작해요.",
        "할 일 목록을 하나씩 지워봐요.",
        "오늘의 나는 어제의 나보다 나아요.",
        "작은 진척도 칭찬받을 자격이 있어요.",
        "오후도 여전히 가능성이 넘쳐요.",
        "지금 집중하면 저녁이 편해요.",
        "중간에 포기하지 않는 게 대단한 거예요.",
        "오늘도 묵묵히 나아가고 있어요.",
        "필요하면 잠깐 자리를 벗어나봐요.",
        "오후의 고요함 속에서 집중해봐요.",
        "지금 이 순간도 나중에 추억이 돼요.",
        "밥 먹고 졸린 건 당연한 거예요.",
        "오늘 오후도 의미 있게 보내고 있어요.",
        "무엇이든 꾸준한 게 제일 강해요.",
        "오늘의 집중이 내일의 여유를 만들어요.",
        "잘 되고 있어요, 믿어봐요.",
        "오후 햇살처럼 따뜻하게 나아가요.",
        "조금만 더 하면 오늘 하루가 뿌듯해져요.",
        "중간에 온 것만으로도 대단해요.",
        "오늘 오후도 당신이 주인공이에요.",
        "꾸준함이 결국 이겨요.",
        "지금 하는 것들이 모두 의미 있어요.",
        "조용히 최선을 다하는 것도 멋있어요.",
        "오늘 오후, 잘 버텨주고 있어요.",
        "조금씩 나아가는 것, 그게 전부예요.",
    ]
}

fn phrases_evening() -> Vec<&'static str> {
    vec![
        "오늘 하루도 수고하셨습니다.",
        "이제 조금 쉬어도 괜찮아요.",
        "저녁 식사는 맛있게 드세요.",
        "오늘 하루도 참 잘 해냈어요.",
        "퇴근 후의 시간은 오롯이 당신의 것이에요.",
        "오늘 하루를 스스로 칭찬해봐요.",
        "저녁 노을처럼 오늘도 멋진 하루였어요.",
        "잠깐 산책이라도 어때요?",
        "오늘 수고한 나에게 작은 선물을 줘봐요.",
        "저녁엔 좋아하는 것을 해봐요.",
        "오늘 이룬 것들을 돌아봐요.",
        "하루의 끝은 항상 달콤해요.",
        "내일을 위해 오늘 잘 쉬어봐요.",
        "저녁 한 잔의 따뜻한 음료 어때요?",
        "오늘 하루도 충실하게 살았어요.",
        "저녁이 있는 삶, 소중히 여겨봐요.",
        "오늘 하루를 마무리할 시간이에요.",
        "천천히 긴장을 풀어봐요.",
        "오늘 고생한 몸과 마음을 달래줘요.",
        "저녁의 고요함을 즐겨봐요.",
        "오늘 하루 잘 달려왔어요.",
        "지금 이 순간을 편안하게 보내봐요.",
        "오늘 하루 당신은 최선을 다했어요.",
        "내일의 나를 위해 오늘 쉬어요.",
        "저녁엔 좋아하는 음악을 들어봐요.",
        "오늘 작은 것들에 감사해봐요.",
        "잘 마무리하는 것도 실력이에요.",
        "오늘 하루도 함께해서 좋았어요.",
        "저녁 하늘처럼 평온한 시간 되세요.",
        "오늘 하루의 마지막을 여유롭게.",
        "저녁엔 잠깐 디지털 기기를 내려놓아봐요.",
        "오늘 하루도 무사히 마쳤어요.",
        "저녁의 여유는 내일의 에너지예요.",
        "오늘 잘한 일 하나를 떠올려봐요.",
        "하루를 마무리하는 이 시간이 소중해요.",
        "오늘도 살아있음에 감사해요.",
        "저녁이 되면 마음이 차분해져요.",
        "오늘 힘든 일이 있었다면, 다 지나갔어요.",
        "잘 쉬어야 내일도 잘 할 수 있어요.",
        "오늘 하루의 마침표를 잘 찍어봐요.",
        "저녁엔 좋아하는 책 한 페이지 어때요?",
        "오늘도 나름대로 잘 해냈어요.",
        "저녁 바람이 시원하게 느껴지면 좋겠어요.",
        "내일은 더 나은 하루가 될 거예요.",
        "오늘의 나를 아껴줘요.",
        "오늘 하루, 정말 수고 많았어요.",
        "저녁엔 맛있는 걸 먹어봐요.",
        "하루가 끝나도 당신은 충분해요.",
        "오늘의 마무리가 내일의 시작이에요.",
        "편안한 저녁 되세요.",
    ]
}

fn phrases_night() -> Vec<&'static str> {
    vec![
        "이 시간에도 깨어있군요, 수고해요.",
        "밤에는 몸이 회복을 원해요.",
        "야식은 조금만 드세요.",
        "오늘 하루도 수고하셨어요.",
        "새벽의 고요함을 즐기고 있나요?",
        "충분한 수면이 내일을 만들어요.",
        "이 시간도 언젠가 소중한 추억이 돼요.",
        "밤하늘을 잠깐 올려다봐요.",
        "너무 무리하지 않아도 괜찮아요.",
        "오늘을 마무리할 준비가 됐나요?",
        "밤이 깊어갈수록 내일이 가까워져요.",
        "지금 하는 일이 마무리되면 꼭 쉬어요.",
        "자기 전에 물 한 잔 마셔봐요.",
        "오늘 하루도 잘 버텼어요.",
        "새벽엔 조용히 자신과 대화해봐요.",
        "밤 시간도 소중하게 써봐요.",
        "지금 집중하는 당신이 멋있어요.",
        "야간에도 최선을 다하는 당신을 응원해요.",
        "조금 있으면 날이 밝아올 거예요.",
        "이 시간의 고요함이 당신만의 것이에요.",
        "밤새 수고하는 당신에게 감사해요.",
        "새벽은 용감한 자들의 시간이에요.",
        "내일의 나를 위해 오늘 무리하지 마요.",
        "지금 잠들어도 괜찮아요.",
        "어두운 밤이 지나면 반드시 새벽이 와요.",
        "밤의 적막 속에서도 당신은 빛나요.",
        "이 시간도 당신의 삶의 일부예요.",
        "오늘의 마지막을 평온하게 마무리해요.",
        "하루의 끝에서도 당신은 충분해요.",
        "내일을 위해 편안히 쉬어봐요.",
        "밤새도록 수고하는 당신을 응원해요.",
        "고요한 밤이 당신을 품어줄 거예요.",
        "이 밤이 지나면 새로운 하루가 와요.",
        "지금 하는 일이 끝나면 꼭 자리에 누워봐요.",
        "몸이 먼저 신호를 보내면 귀 기울여줘요.",
        "새벽 공기는 특별히 차갑고 맑아요.",
        "오늘도 어딘가에서 누군가 응원하고 있어요.",
        "밤이 길어도 결국 아침이 와요.",
        "지금 이 시간도 당신에게 의미 있어요.",
        "조용한 밤, 마음도 조용히 가라앉혀봐요.",
        "내일의 에너지를 위해 지금 쉬어봐요.",
        "밤에는 꿈을 꾸는 것도 일이에요.",
        "오늘 하루, 정말 많이 수고했어요.",
        "달빛 아래서도 당신은 열심히 살고 있어요.",
        "이 새벽도 결국 지나가요.",
        "잘 마무리하고 편안히 눈 감아봐요.",
        "늦은 밤까지 함께해줘서 고마워요.",
        "오늘도 끝까지 잘 버텼어요.",
        "이제 쉬어도 충분히 잘 한 거예요.",
        "좋은 밤 되세요.",
    ]
}

// ─────────────────────────────────────────────
// 특수 문구 (시간대별, 확률적으로 등장)
// ─────────────────────────────────────────────

fn special_morning() -> Vec<&'static str> {
    vec![
        "☀️ 오늘은 왠지 특별한 하루가 될 것 같아요!",
        "🌅 아침 햇살이 유난히 아름다운 날이에요.",
        "🐦 창밖에서 새소리가 들리지 않나요?",
        "🍳 오늘 아침은 뭔가 특별한 걸 먹어봐요.",
        "✨ 오늘 하루, 기억에 남을 것 같은 예감이에요.",
    ]
}

fn special_afternoon() -> Vec<&'static str> {
    vec![
        "🌤️ 오늘 오후는 유난히 맑고 좋은 날씨예요.",
        "☕ 오늘은 특별히 맛있는 커피 한 잔 어때요?",
        "🎵 지금 이 순간, 좋아하는 노래 한 곡 틀어봐요.",
        "🌟 오늘 오후엔 뭔가 좋은 일이 생길 것 같아요.",
        "💡 오늘 오후, 갑자기 좋은 아이디어가 떠오를지도!",
    ]
}

fn special_evening() -> Vec<&'static str> {
    vec![
        "🌇 오늘 저녁노을이 유독 예쁜 것 같아요.",
        "🍽️ 오늘 저녁은 좋아하는 음식을 먹어봐요.",
        "🌙 특별히 평화로운 저녁이 되길 바라요.",
        "🎉 오늘 하루, 정말 잘 해냈어요! 스스로 축하해줘요.",
        "🌟 오늘 저녁은 뭔가 특별한 일이 생길 것 같은 예감.",
    ]
}

fn special_night() -> Vec<&'static str> {
    vec![
        "🌌 밤하늘의 별이 오늘따라 더 밝게 빛나요.",
        "🦉 이 새벽, 특별히 당신을 응원하고 싶어요.",
        "🌙 달빛이 유난히 아름다운 밤이에요.",
        "✨ 이 고요한 밤, 작은 소원을 빌어봐요.",
        "🌠 별똥별이 지나간 것 같은, 특별한 새벽이에요.",
    ]
}

// ─────────────────────────────────────────────
// 앱 상태
// ─────────────────────────────────────────────

struct LumiClock {
    // 일반 문구 셔플 덱 (시간대별)
    deck_morning: Vec<&'static str>,
    deck_afternoon: Vec<&'static str>,
    deck_evening: Vec<&'static str>,
    deck_night: Vec<&'static str>,

    // 특수 문구 셔플 덱
    sdeck_morning: Vec<&'static str>,
    sdeck_afternoon: Vec<&'static str>,
    sdeck_evening: Vec<&'static str>,
    sdeck_night: Vec<&'static str>,
    
    adeck_morning: Vec<&'static str>,
    adeck_afternoon: Vec<&'static str>,
    adeck_evening: Vec<&'static str>,
    adeck_night: Vec<&'static str>,

    current_avatar: String,

    // 현재 표시 문구
    current_phrase: String,

    current_weather: String,

    // 마지막 갱신 시각
    last_update: Instant,
}

impl LumiClock {
    fn new() -> Self {
        let mut rng = thread_rng();

        let mut dm = phrases_morning();
        let mut da = phrases_afternoon();
        let mut de = phrases_evening();
        let mut dn = phrases_night();
        dm.shuffle(&mut rng);
        da.shuffle(&mut rng);
        de.shuffle(&mut rng);
        dn.shuffle(&mut rng);

        let mut sm = special_morning();
        let mut sa = special_afternoon();
        let mut se = special_evening();
        let mut sn = special_night();
        sm.shuffle(&mut rng);
        sa.shuffle(&mut rng);
        se.shuffle(&mut rng);
        sn.shuffle(&mut rng);

        // 초기 문구: 현재 시간대 첫 번째
        let hour = Local::now().hour();
        let slot = TimeSlot::from_hour(hour);
        let initial = match slot {
            TimeSlot::Morning => dm.last().copied().unwrap_or(""),
            TimeSlot::Afternoon => da.last().copied().unwrap_or(""),
            TimeSlot::Evening => de.last().copied().unwrap_or(""),
            TimeSlot::Night => dn.last().copied().unwrap_or(""),
        };
        
        let mut am = avatars_morning();
        let mut aa = avatars_afternoon();
        let mut ae = avatars_evening();
        let mut an = avatars_night();
        am.shuffle(&mut rng);
        aa.shuffle(&mut rng);
        ae.shuffle(&mut rng);
        an.shuffle(&mut rng);

        let initial_avatar = match slot {
            TimeSlot::Morning => am.last().copied().unwrap_or("(=^･ω･^=)"),
            TimeSlot::Afternoon => aa.last().copied().unwrap_or("( •̀ ω •́ )✧"),
            TimeSlot::Evening => ae.last().copied().unwrap_or("(￣▽￣)"),
            TimeSlot::Night => an.last().copied().unwrap_or("(；￣Д￣)"),
        };

        Self {
            deck_morning: dm,
            deck_afternoon: da,
            deck_evening: de,
            deck_night: dn,
            sdeck_morning: sm,
            sdeck_afternoon: sa,
            sdeck_evening: se,
            sdeck_night: sn,
            adeck_morning: am,
            adeck_afternoon: aa,
            adeck_evening: ae,
            adeck_night: an,
            current_avatar: initial_avatar.to_string(),
            current_phrase: initial.to_string(),
            current_weather: get_mock_weather(),
            last_update: Instant::now(),
        }
    }

    /// 5분 주기마다 호출: 현재 시간대의 덱에서 팝. 10% 확률로 특수 문구.
    fn pick_next_phrase(&mut self) {
        let mut rng = thread_rng();
        let hour = Local::now().hour();
        let slot = TimeSlot::from_hour(hour);

        // 10% 확률로 특수 문구 시도
        let use_special: f64 = rand::random();
        if use_special < 0.10 {
            let sdeck = match slot {
                TimeSlot::Morning => &mut self.sdeck_morning,
                TimeSlot::Afternoon => &mut self.sdeck_afternoon,
                TimeSlot::Evening => &mut self.sdeck_evening,
                TimeSlot::Night => &mut self.sdeck_night,
            };
            if sdeck.is_empty() {
                // 소진 시 재셔플
                let mut fresh = match slot {
                    TimeSlot::Morning => special_morning(),
                    TimeSlot::Afternoon => special_afternoon(),
                    TimeSlot::Evening => special_evening(),
                    TimeSlot::Night => special_night(),
                };
                fresh.shuffle(&mut rng);
                *sdeck = fresh;
            }
            if let Some(p) = sdeck.pop() {
                self.current_phrase = p.to_string();
                return;
            }
        }

        // 일반 문구
        let deck = match slot {
            TimeSlot::Morning => &mut self.deck_morning,
            TimeSlot::Afternoon => &mut self.deck_afternoon,
            TimeSlot::Evening => &mut self.deck_evening,
            TimeSlot::Night => &mut self.deck_night,
        };
        if deck.is_empty() {
            // 전체 소진 시 재셔플
            let mut fresh = match slot {
                TimeSlot::Morning => phrases_morning(),
                TimeSlot::Afternoon => phrases_afternoon(),
                TimeSlot::Evening => phrases_evening(),
                TimeSlot::Night => phrases_night(),
            };
            fresh.shuffle(&mut rng);
            *deck = fresh;
        }
        if let Some(p) = deck.pop() {
            self.current_phrase = p.to_string();
        }
        
        // 아바타도 함께 교체
        let adeck = match slot {
            TimeSlot::Morning => &mut self.adeck_morning,
            TimeSlot::Afternoon => &mut self.adeck_afternoon,
            TimeSlot::Evening => &mut self.adeck_evening,
            TimeSlot::Night => &mut self.adeck_night,
        };
        if adeck.is_empty() {
            let mut fresh = match slot {
                TimeSlot::Morning => avatars_morning(),
                TimeSlot::Afternoon => avatars_afternoon(),
                TimeSlot::Evening => avatars_evening(),
                TimeSlot::Night => avatars_night(),
            };
            fresh.shuffle(&mut rng);
            *adeck = fresh;
        }
        if let Some(a) = adeck.pop() {
            self.current_avatar = a.to_string();
        }

        self.current_weather = get_mock_weather();
    }
}

// ─────────────────────────────────────────────
// eframe App
// ─────────────────────────────────────────────

impl eframe::App for LumiClock {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(Duration::from_millis(200));

        // Esc로 닫기
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // 5분 경과 시 문구 교체
        if self.last_update.elapsed() >= Duration::from_secs(300) {
            self.pick_next_phrase();
            self.last_update = Instant::now();
        }

        let now = Local::now();
        let hour = now.hour();
        let slot = TimeSlot::from_hour(hour);
        let avatar = self.current_avatar.clone();

        let time_text = now.format("%H:%M:%S").to_string();
        let date_text = format!("{} {}", now.format("%Y-%m-%d"), korean_weekday(now.weekday()));
        let phrase = self.current_phrase.clone();
        let weather_str = format!("LumiClock 1.0.1b Weather : {}", self.current_weather);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(egui::Color32::from_rgba_unmultiplied(17, 19, 24, 150))
                    .stroke(egui::Stroke::new(
                        1.0_f32,
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 30),
                    )),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(14.0);
                    ui.label(
                        egui::RichText::new(time_text)
                            .size(44.0)
                            .strong()
                            .color(egui::Color32::from_rgb(143, 156, 255)),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(date_text)
                            .size(14.0)
                            .color(egui::Color32::from_rgb(170, 179, 197)),
                    );
                    ui.add_space(16.0);
                    ui.label(egui::RichText::new(avatar).size(28.0));
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(phrase)
                            .size(16.0)
                            .color(egui::Color32::WHITE)
                            .strong(),
                    );
                    ui.add_space(14.0);
                    ui.label(
                        egui::RichText::new(weather_str)
                            .size(12.0)
                            .color(egui::Color32::from_rgb(170, 179, 197)),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Esc: 닫기")
                            .size(10.0)
                            .color(egui::Color32::from_rgb(110, 118, 135)),
                    );
                });
            });
    }
}

// ─────────────────────────────────────────────
// main
// ─────────────────────────────────────────────

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("LumiClock")
            .with_inner_size([380.0, 240.0])
            .with_min_inner_size([320.0, 220.0])
            .with_always_on_top() // 이미 존재하는 최상단 고정 기능
            .with_transparent(true) // 글래스 효과를 위해 반드시 추가해야 함!
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "LumiClock",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "noto".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/LINESeedKR-Rg.otf")).into(),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "noto".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, "noto".to_owned());
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(LumiClock::new()))
        }),
    )
}