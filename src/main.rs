use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let mut file = File::create("public/App.jsx").unwrap();
    writeln!(&mut file, "const App = () => {{").unwrap();
    writeln!(&mut file, "const [count, setCount] = useState(0)").unwrap();
    writeln!(&mut file, "return <><Component1 onClick={{()=>setCount(count + 1)}} />{{count}}</>").unwrap();
    writeln!(&mut file, "}}").unwrap();
    let end = 101;
    for n in 1..end {
        let code = format!("const Component{} = (props) => {{", n);
        writeln!(&mut file, "{}", code).unwrap();
        
        if(n == end-1){
            writeln!(&mut file, "return <button onClick={{props.onClick}}>click me</button>").unwrap();

        writeln!(&mut file, "}}").unwrap();
        }else{
            let call = format!("return <Component{} onClick={{props.onClick}} />", n+1);
            writeln!(&mut file, "{}", call).unwrap();
            writeln!(&mut file, "}}").unwrap();
        }
        }
}
