// routes
import { useNavigate } from 'react-router-dom';
import { useEffect } from 'react';
import Router from './routes';
// theme
import ThemeProvider from './theme';
// components
import ScrollToTop from './components/ScrollToTop';
import { BaseOptionChartStyle } from './components/chart/BaseOptionChart';

import 'regenerator-runtime/runtime';
import { useDispatch } from 'react-redux';
import { SET_PROFILE } from './redux/slices/user';

// ----------------------------------------------------------------------

export default function App() {
  const navigate = useNavigate();
  const dispatch = useDispatch();

  const getProfile = async () => {
    const walletAccountId = walletConnection.getAccountId();
    const account = await window.nearConnection.account(walletAccountId);
    const balance = await account.getAccountBalance();
    const dataToSend = {
      walletAddress: walletAccountId,
      balance,
      isConnected: true,
    };
    dispatch(SET_PROFILE(dataToSend));
  };

  useEffect(() => {
    if (window.walletConnection && window.walletConnection.isSignedIn()) {
      navigate('/dashboard/products', { repalce: true });
      getProfile();
    } else {
      navigate('/login');
    }
  }, []);

  return (
    <ThemeProvider>
      <ScrollToTop />
      <BaseOptionChartStyle />
      <Router />
    </ThemeProvider>
  );
}
