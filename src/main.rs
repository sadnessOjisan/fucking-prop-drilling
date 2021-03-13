use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("public/index.jsx").unwrap();
    writeln!(&mut file, "import React from 'react'").unwrap();
    writeln!(&mut file, "import ReactDOM from 'react-dom'").unwrap();
    writeln!(&mut file, "const App = () => {{").unwrap();
    writeln!(&mut file, "const [count, setCount] = React.useState(0)").unwrap();
    writeln!(
        &mut file,
        "return <>{{count}}<Component1 onClick={{()=>setCount(count + 1)}} /></>"
    )
    .unwrap();
    writeln!(&mut file, "}}").unwrap();
    let end = 300000;
    for n in 1..end {
        let code = format!("const Component{} = (props) => {{", n);
        writeln!(&mut file, "{}", code).unwrap();
        writeln!(&mut file, "const count = {}", n).unwrap();
        if (n == end - 1) {
            writeln!(
                &mut file,
                "return <button onClick={{props.onClick}}>click me</button>"
            )
            .unwrap();
            writeln!(&mut file, "}}").unwrap();
        } else {
            let call = format!(
                "return <div style={{{{paddingBottom: 1, backgroundColor: count%4===0?'red':count%4===1?'blue':count&4?'green':'yellow'}}}}><Component{} onClick={{props.onClick}} /></div>",
                n + 1
            );
            writeln!(&mut file, "{}", call).unwrap();
            writeln!(&mut file, "}}").unwrap();
        }
    }
    writeln!(
        &mut file,
        "ReactDOM.render(<App />, document.getElementById('root'))"
    )
    .unwrap();
}
