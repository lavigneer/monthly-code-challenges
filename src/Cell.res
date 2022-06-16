%%raw(`import './App.css'`)

@react.component
let make = (~character: option<string>) => {
    switch character {
        | None => <div className="empty-cell"></div>
        | Some(char) => <div className="filled-cell">{React.string(char)}</div>
    }
}
