import { createSlice } from '@reduxjs/toolkit';

const initialState = {
  list: [
    {
      id: '',
      name: '',
      quantity: 0,
      price: 0,
      quality: {
        certificate: [],
        stage: [],
      },
      ownerId: '',
    },
  ],
};

export const productSlice = createSlice({
  name: 'product',
  initialState,
  reducers: {
    SET_PROFILE: (state, action) => {
      Object.assign(state, action.payload);
    },
  },
});

export const { SET_PROFILE } = productSlice.actions;

export default productSlice.reducer;
