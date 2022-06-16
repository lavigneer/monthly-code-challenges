%%raw(`import './App.css'`)

@val @scope("window")
external addEventListener: (string, unit => unit) => unit = "addEventListener"

@val @scope("window")
external removeEventListener: (string, unit => unit) => unit = "removeEventListener"

type action = KeyPress(string)
type state = {
  guesses: Js.Array2.t<Js.Array2.t<char>>,
  row: int,
  col: int,
  win: bool
}

let initialState = {
  row: 0,
  col: 0,
  guesses: [[]],
  win: false
}

let reducer = (state, action) => {
  switch action {
    | KeyPress("Enter") => {...state, row: state.row + 1}
    | _ => state
  }
}

@react.component
let make = () => {
  let (state, dispatch) = React.useReducer(reducer, initialState)

  let handleKeyPress = React.useCallback1((evt) => {
      let key = ReactEvent.Keyboard.key(evt)
      dispatch(KeyPress(key))
  }, [dispatch])

  <div onKeyPress={handleKeyPress}><Cell character={None}/></div>
}
