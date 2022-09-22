import { createSlice } from '@reduxjs/toolkit';

const initialState = {
  walletAddress: '',
  username: '',
  balance: 0,
  isConnected: false,
};

export const userSlice = createSlice({
  name: 'user',
  initialState,
  reducers: {
    setConnection: (state, action) => {
      state.connected = action.payload;
    },
    setWalletAddress: (state, action) => {
      state.walletAddress = action.payload;
    },
    setUsername: (state, action) => {
      state.username = action.payload;
    },
    setBalance: (state, action) => {
      state.balance = action.payload;
    },
  },
});

export const { setConnection, setWalletAddress, setUsername, setBalance } = userSlice.actions;

export default userSlice.reducer;
