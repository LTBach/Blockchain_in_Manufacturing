// @mui
import { Button, ButtonGroup, Stack } from '@mui/material';
// components
import { useSnackbar } from 'notistack';
import { useDispatch } from 'react-redux';
import { useNavigate } from 'react-router-dom';
import { ethers } from 'ethers';
import { setBalance, setConnection, setWalletAddress } from '../../../redux/slices/user';
import Iconify from '../../../components/Iconify';

// ----------------------------------------------------------------------

export default function LoginForm() {
  const navigate = useNavigate();
  const { enqueueSnackbar } = useSnackbar();
  const dispatch = useDispatch();

  return (
    <Stack sx={{ margin: '16px 0' }} alignItems="center">
      <ButtonGroup orientation="vertical" aria-label="vertical outlined button group">
        <Button variant="outlined" size="large">
          Connect to NEAR
        </Button>
      </ButtonGroup>
    </Stack>
  );
}
