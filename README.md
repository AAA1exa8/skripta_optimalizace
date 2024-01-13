# Preface
Autor basically neumí česky a bude v textu používat mnoho anglizmů a občas i zkombinuje češtinu a angličitnu do jedné věty a předem se omlouvá komukoliv, kdo  tím  bude uražen.

# Cíl

Cílem těchto skript je vysvětlit běžné koncepty v optimalizaci a představit nejčastější způsoby optimalizace. 

# Definice

> Optimalizace v kontextu software je proces při kterém zvyšujeme efektivitu využití prostředků.
> Prostředky jsou obvykle myšleny processor cycles nebo memory

# Proč Optimalizujeme

Pokud jste přesvědčeni, že optimalizace má smysl, tak tuto část přeskočte.

> "Computers are fast why should we optimize when everything runs so fast"
- some idiot on the internet

Rychlost našeho software má přímý dopad na UX koncového uživatele. Přes rychlost počítačů jsme schopni se špatně optimalizovaným kódem vytvořit pomalé a zasekané aplikace. 
React aplikace bez optimalizace bude od určité komplexity bude provádět zbytečné rerenders DOM. To je případ aplikace v high level jazyce běžící ve web browseru na desktopovém operačním systému. Fakt že tato aplikace vůbec běží, je výsledkem obrovského množství optimalizace react projektu, browser engines, V8 runtime a samozřejmě vašeho operačního systému.
Zároveň v dnešní době často budete používat FaaS, Paas nebo IaaS účtující si pouze za zdroje které používáte. Pokud používáte méně zdrojů platíte méně.

# Proč je kód pomalý?

Odpověď na tuto složitou otázku záleží na vašem konkrétním software. Lze indentifikovat některá místa která můžou váš kód zpomalovat

1. Váš kód je špatně designed a dělá víc práce než je potřeba
	1. Do této kategorie bude zapadat obrovská škála problémů které se týkají designu vašeho software (například zda si vyberete p2p komunikaci nebo client-server a další designová rozhodnutí)
	2. váš kód dělá znova práci, kterou už udělal
		1. Minimax bez caching, který počítá po každém kole znova všechno do nějáké hloubky
		2. Děláte requesty pro data, která už máte
	3. váš kód dělá práci, kterou není potřeba udělat
		1. Základní minimax vypočítává hodnoty, které se dají eliminovat a nemusejí být za určitých podmínek počítány
		2. Kvůli špatnému designu API musíte dělat víc requestů pro data, která by dávalo smysl získávat v rámci jedné query
3. Špatné algoritmy a datové struktury
	1. Nutnost výběru datové struktury s ideálním look-up time podle počtu objektů vložených do kolekce nebo tree.
	2. Někdy můžete mít algoritmus s lepším O(), který ale vyžaduje více paměti a může být nevhodný pro systémy kde je memory spare resource
4. Špatná práce s pamětí
	1. Zbytečně alokujete paměť
	2. Využíváte paměť neefektivně
5. Source code level
	1. Obvykle nezpomaluje vaši aplikaci. Zpomalení na source code level je když je váš kód zpomalen rozdíly mezi ++i a i++ nebo while(1) vs for(;;)

# Techniky optimalizace

Dále uvedené techniky řeší výše uvedené problémy. Nebudu se věnovat optimalizaci na designové úrovni protože ta obvykle vyžaduje specifiackou znalost vašeho problému a je nejtěžší na generalizaci.

### Vybrání správné datové struktury

Pro vybrání nejlepší datové struktury idealní datové struktury je nejlepší v svém řešení postupně vyzkoušet všechny struktury, které připadají v úvahu.
Zde ukážu případ kde je Vector lepší v situaci v které by většina lidí očekávala že bude LinkedLIst lepší
(probléím je vzatý z keynote od Bjarne Stroustrup na GoingNative 2012 na téme proč jsou linked struktury špatné pro performance [link](https://www.youtube.com/watch?v=YQs6IC-vgmo))
Problém vypadá takto:
1. Vygeneruj N náhodných čísel a postupně je vkládej do kolekce tak aby byly vždy v pořadí. Pokud vygenerujeme 5 1 4 2 dostaneme:
	- 5
	- 1 5
	- 1 4 5
	- 1 2 4 5
2. Odstraňuj v náhodném pořadí jeden element po druhém dokud v kolekci nezbydou žádné elementy. Pro pozice 1 2 0 0 dostaneme:
	 - 1 2 4 5
	 - 1 4 5
	 - 1 4
	 - 4
3. Pro jaké N je lepší použít linked list radši než vector 
Většina programátorů by očekávala že linked list bude pro situaci v které probíhá tak velké množství insercí a mazání elementů mnohem lepší. Co se ale z výsledku následujícího programu dozvíme že čas který musíme strávit realokacema vektoru je menší než čas strávený vyhledáváním kam má proběhnou inserce elementu do listu. V vektoru zjistíme kam insertnout v $O(log(n))$ zatímco v listu $O(n)$ protože linkedlist nedovoluje random access. Zároveň s linked list máme mnohem více cache misses.

```rust
fn fill_data_structures(length: usize) -> (Vec<usize>, LinkedList<usize>, u128, u128) {  
    // initialize data structures  
    let mut vec = Vec::new();  
    let mut list = LinkedList::new();  
    // generate random values  
    let rng = generate_values(length);  
    // create two copies of rng in because of borrow checker  
    let rng2 = rng.clone();  
  
    // start the timer for vector insertion  
    let start_vec = Instant::now();  
    for i in rng {  
        // search the position to insert the value  
        match vec.binary_search(&i) {  
            Ok(pos) | Err(pos) => vec.insert(pos, i),  
        }    }    // stop the timer  
    let vec_insert_time = start_vec.elapsed().as_millis();  
  
    // start the timer for list insertion  
    let start_list = Instant::now();  
    for i in rng2 {  
        // get cursor to the front of the list  
        let mut cursor = list.cursor_front_mut();  
        // iterate trough the list until the value is greater than the current value  
        loop {  
            match cursor.current() {  
                Some(val) if *val < i => {  
                    cursor.move_next();  
                }                _ => break,  
            }        }        // insert the value into the list  
        cursor.insert_before(i);  
    }    // stop the timer  
    let list_insert_time = start_list.elapsed().as_millis();  
    // return the data structures and the times  
    (vec, list, vec_insert_time, list_insert_time)  
}  
  
fn remove_elements(  
    mut vec: Vec<usize>,  
    mut list: LinkedList<usize>,  
    removes: Vec<usize>,  
) -> (u128, u128) {  
    // start the timer for vector removal  
    let start_vec = Instant::now();  
    for i in &removes {  
        // remove the value in the index of i  
        vec.remove(*i);  
    }    // stop the timer  
    let duration_vec = start_vec.elapsed().as_millis();  
  
    // start the timer for list removal  
    let start_list = Instant::now();  
    for i in &removes {  
        // remove the value in the index of i  
        list.remove(*i);  
    }    // stop the timer  
    let duration_list = start_list.elapsed().as_millis();  
    // return the times  
    (duration_vec, duration_list)  
}  
  
fn main() {  
    // the starting length of the data structures  
    let mut length = 10;  
    // create the csv file  
    let mut wtr = Writer::from_path("times.csv").unwrap();  
    wtr.write_record([  
        "length",  
        "vec_insert_time",  
        "list_insert_time",  
        "vec_remove_time",  
        "list_remove_time",  
    ])    .unwrap();  
  
    for _ in 0..7 {  
        // get data structures filled with sorted random values and times it took to fill them  
        let (vec, list, vec_insert_time, list_insert_time) = fill_data_structures(length);  
        // generate indices to remove  
        let removes = generate_indices(length);  
        // get times it took to remove elements  
        let (vec_remove_time, list_remove_time) = remove_elements(vec, list, removes);  
        // write the times to csv  
        wtr.write_record(&[  
            length.to_string(),  
            vec_insert_time.to_string(),  
            list_insert_time.to_string(),  
            vec_remove_time.to_string(),  
            list_remove_time.to_string(),  
        ])        .unwrap();  
        // increase the length of the data structures and repeat  
        length *= 10;  
    }    // flush the csv file  
    wtr.flush().unwrap();  
}
```
Toto je snippet z [[list/src/main.rs]]


### Vybrání rychlejšího aloritmu

Rychlost algoritmu obvykle učujeme pomocí big O notace ($O(n)$, $O(n^2)$...) pokud víte že se dá $O()$ vašeho kódu snížit nebo že existuje algoritmus který dělát to samé s menším $O()$ tak je nejlepší ho otestovat že doopravdy vede k zvýšení výkonu a poté aplikovat.
Zde dám jako příklad implementaci algoritmu minimax a alpha-beta pruning. Kde [minimax](https://en.wikipedia.org/wiki/Minimax) projde všechny možnosti do určité hloubky i kdyby tam možnost zaručeně vedla k výsledku co se nepoužije. Oproti tomu použiji [alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning). Minimax má časovou komplexitu $O(b^d)$ kde b je branching factor v každém kole hry a d je depth do kterého hru zkoumáme. Mezitím alpha-beta pruning časovou komplexitu sníží na $O(\sqrt{b^d})$.
(toto je implementace minimaxu určená k hraní hry [kalaha](https://en.wikipedia.org/wiki/Kalah))

zde je základní minimax
```rust
fn minimax(node: &Kalah, depth: u64, maximizing_player: bool) -> (i32, usize) {  
    if depth == 0 || node.game_over() {  
        let result = (node.heuristic(), 0);  
        return result;  
	}  
    if maximizing_player {  
        let mut max_eval = i32::MIN;  
        let mut best_move = 0;  
        for (i, (child, _)) in node.get_children().iter().enumerate() {  
            let (eval, _) = minimax(child, depth - 1, false);  
            if eval > max_eval {  
                max_eval = eval;  
                best_move = i;  
            }        
        }
        (max_eval, best_move)  
    } else {  
        let mut min_eval = i32::MAX;  
        let mut best_move = 0;  
        for (i, (child, _)) in node.get_children().iter().enumerate() {  
            let (eval, _) = minimax(child, depth - 1, true);  
            if eval < min_eval {  
                min_eval = eval;  
                best_move = i;  
            }        
        }        
        (min_eval, best_move)  
    }
}
```

zde je alpha-beta pruning který přeskakuje hodnoty u kterých jsme schopní určit že nepovedou k lepší hodnotě
```rust
fn alpha_beta_pruning(node: &Kalah, depth: u64, alpha: i32, beta: i32, maximizing_player: bool) -> (i32, usize) {  
    if depth == 0 || node.game_over() {  
        let result = (node.heuristic(), 0);  
        return result;  
    }    
    if maximizing_player {  
        let mut max_eval = i32::MIN;  
        let mut best_move = 0;  
        let mut alpha = alpha;  
        for (i, (child, _)) in node.get_children().iter().enumerate() {  
            let (eval, _) = minimax(child, depth - 1, alpha, beta, false);  
            if eval > max_eval {  
                max_eval = eval;  
                best_move = i;  
            }            alpha = max(alpha, eval);  
            if beta <= alpha {  
                break;  
            }        
        }        
        (max_eval, best_move)  
    } else {  
        let mut min_eval = i32::MAX;  
        let mut best_move = 0;  
        let mut beta = beta;  
        for (i, (child, _)) in node.get_children().iter().enumerate() {  
            let (eval, _) = minimax(child, depth - 1, alpha, beta, true);  
            if eval < min_eval {  
                min_eval = eval;  
                best_move = i;  
            }            beta = min(beta, eval);  
            if beta <= alpha {  
                break;  
            }        
        }        
        (min_eval, best_move)  
    }
}
```

### Caching

Caching je když ukládáme již jednou udělanou práci abychom ji nemuseli dělat znova. V přechozím příkladu si můžeme všimnou že spočítáme skoro všechny hodnoty do nějáké hloubky a poté co algoritmus zahraje tah a hráč zahraje tah začne algoritmus počítat znova všechny tyto tahy jen o jednu hloubku víc. Pokud přidáme cache můžeme ukládat hotovou práci a nemusíme počítat tolik hodnot znova.

```rust
fn minimax(node: &Kalah, depth: u64, alpha: i32, beta: i32, maximizing_player: bool, cache: &mut LruCache<Kalah, (i32, usize)>) -> (i32, usize) {  
    if let Some(&(score, move_)) = cache.get(node) {  
        return (score, move_);  
    }    
    if depth == 0 || node.game_over() {  
        let result = (node.heuristic(), 0);  
        cache.put(node.clone(), result);  
        return result;  
    }    
    if maximizing_player {  
        let mut max_eval = i32::MIN;  
        let mut best_move = 0;  
        let mut alpha = alpha;  
        for (i, (child, _)) in node.get_children().iter().enumerate() {  
            let (eval, _) = minimax(child, depth - 1, alpha, beta, false, cache);  
            if eval > max_eval {  
                max_eval = eval;  
                best_move = i;  
            }            alpha = max(alpha, eval);  
            if beta <= alpha {  
                break;  
            }        
        }        
        let result = (max_eval, best_move);  
        cache.put(node.clone(), result);  
        result  
    } else {  
        let mut min_eval = i32::MAX;  
        let mut best_move = 0;  
        let mut beta = beta;  
        for (i, (child, _)) in node.get_children().iter().enumerate() {  
            let (eval, _) = minimax(child, depth - 1, alpha, beta, true, cache);  
            if eval < min_eval {  
                min_eval = eval;  
                best_move = i;  
            }            beta = min(beta, eval);  
            if beta <= alpha {  
                break;  
            }        
        }        
        let result = (min_eval, best_move);  
        cache.put(node.clone(), result);  
        result  
    }  
}
```


### Zvýšení efektivity algoritmu

Toto je proces při kterém zmenšujeme množství toho co algoritmus dělá aniž bychom snížily časovou komplexitu. Tedy že vynecháváme zbytečnou práci.
Ukázka této optimalizace je na problému z [Advent of Code 2022 den 6](https://adventofcode.com/2022/day/6) part 2 a kód je od [david-a-perez](https://gist.github.com/david-a-perez/067a126edf72bbca9325adaa8e53769a) (pokud se chcete dozvědět více o tomto kódu doporučuji toto [video](https://www.youtube.com/watch?v=U16RnpV48KQ)). Cílem je nalézt první pozici v stringu na které se nachází 14 různých charakterů.

Toto je naivní implementace řešení
```rust
fn hashset(i: &[u8]) -> usize {  
    return i  
        .windows(14)  
        .position(|w| {  
	        // iterate over all chars and collect the to HashSet
            return w.iter().collect::<HashSet<_>>().len() == 14;  
        })        .map(|x| x + 14)  
        .unwrap();  
}
```

V tomto řešení vrátíme false hned jak narazíme na duplikát čímž přeskakujeme zbytečnou práci
```rust
fn hash_faster(i: &[u8]) -> usize {  
    return i  
        .windows(14)  
        .position(|w| {  
            let mut hash = HashSet::new();  
            for x in w {  
                if !hash.insert(x) {  
	                // skip immideataly to the next window
                    return false;  
                }            
            }            
            true  
        })  
        .map(|x| x + 14)  
        .unwrap();  
}
```

Tady autor masivně zrychlil operace kontroly počtu charakterů a "vkládání" charakterů do array
```rust
pub fn benny(input: &[u8]) -> Option<usize> {  
	// the filter works as array of true or false
	// each bit is wether char on of the letters in the alfabet is in the window
    let mut filter = 0u32;  
    input  
        .iter()  
        .take(14 - 1)  
        // toggle the first 13 bits
        .for_each(|c| filter ^= 1 << (c % 32));  
    input.windows(14).position(|w| {  
        let first = w[0];  
        let last = w[w.len() - 1];
        // toggle the char of the window as present in the filter   
        filter ^= 1 << (last % 32);  
        // count amount of distincr chars
        let res = filter.count_ones() == 14 as _;  
        // toggle the first char of the window out of the window
        filter ^= 1 << (first % 32);  
        res  
    })  
}
```


Zde je díky reverzní iteraci je autor schopný přeskočit velkou část práce.
```rust
pub fn nerd_face(input: &[u8]) -> Option<usize> {  
    let mut idx = 0;  
    while let Some(slice) = input.get(idx..idx + 14) {  
        let mut state = 0u32;  
  
        if let Some(pos) = slice.iter().rposition(|byte| {  
            let bit_idx = byte % 32;  
            let ret = state & (1 << bit_idx) != 0;  
            state |= 1 << bit_idx;  
            ret  
        }) {  
            idx += pos + 1;  
        } else {  
            return Some(idx);  
        }    
    }    
    return None;  
}
```

### Ždímání instrukcí

Toto jsou optimalizace na úrovni assembly. Na dělání těchto optimalizací je potřeba hodně znalostí assembly a technologií které používáte.
Zde jsou materiály pokud se o tématu chcete dozvědět více:
https://www.youtube.com/watch?v=QGYvbsHDPxo
https://www.youtube.com/watch?v=4LiP39gJuqE
https://xuanwo.io/2023/04-rust-std-fs-slower-than-python/

# Závěr

Až na poslední techniku optimalizace jsou všechny tyto techinky možné v všech reálných jazycích. 
Důležité je si uvědomit jeslti je potřeba váš kód optimalizovat a jestli neděláte premature optimization (pro více info se koukněte [zde](https://www.youtube.com/watch?v=tKbV6BpH-C8))
