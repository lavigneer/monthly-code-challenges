%%raw(`import './App.css'`)

type action = KeyPress(string)
type state = {
  guesses: Js.Array2.t<Js.Array2.t<char>>,
  row: int,
  col: int,
  win: bool,
}

let numCharactersPerWord = 5

let initialState = {
  row: 0,
  col: 0,
  guesses: [[]],
  win: false,
}

let isAlphaChar = ch => {
  let code = Js.String2.charCodeAt(ch, 0)
  (code >= 65.0 && code <= 90.0) || (code >= 97.0 && code <= 122.0)
}

let reducer = (state, action) => {
  switch action {
  | KeyPress("Enter") => {
      ...state,
      row: state.row + 1,
      guesses: Js.Array2.concat(state.guesses, []),
    }
  | KeyPress(key) if isAlphaChar(key) =>
    state.guesses[state.row][state.col] = key->String.get(0)
    {...state, col: Js.Math.min_int(numCharactersPerWord, state.col + 1)}
  | _ => state
  }
}

@react.component
let make = () => {
  let containerEl = React.useRef(Js.Nullable.null)
  let (state, dispatch) = React.useReducer(reducer, initialState)

  Js.Console.log(state)

  let handleKeyPress = React.useCallback1(evt => {
    let key = evt.key
    dispatch(KeyPress(key))
  }: evt: WebApi.Dom.KeyboardEvent.t => unit, [dispatch])

  React.useEffect1(() => {
    WebApi.Dom.Document->WebApi.Dom.Document.addEventListener("click", handleKeyPress)
  }, [handleKeyPress])

  <div ref={ReactDOM.Ref.domRef(containerEl)} onKeyPress={handleKeyPress}>
    <Cell character={None} />
  </div>
}
