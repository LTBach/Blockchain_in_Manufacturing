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
    SET_PROFILE: (state, action) => {
      Object.assign(state, action.payload);
    },
  },
});

export const { SET_PROFILE } = userSlice.actions;

export default userSlice.reducer;
