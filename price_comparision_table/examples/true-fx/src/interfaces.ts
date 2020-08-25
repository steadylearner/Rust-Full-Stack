interface FXRowData {
  data: FXRow;
}

interface FXRow {
    currencyPair: string;
    timestamp: number;
    bidBig: number;
    bidPips: number;
    offerBig: number;
    offerPips: number;
    high: number;
    low: number;
    open: number;
}

interface FXRowState {
  direction?: number;
  changed?: boolean;
}

interface DirectionValue {
  val: number;
}

interface FXComponentState {
  fxRates: FXRow[];
}

