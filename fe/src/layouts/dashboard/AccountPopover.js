import { useRef, useState } from 'react';
import { Box, Dialog, DialogTitle, IconButton, Menu, MenuItem, DialogContent, Typography } from '@mui/material';
import PersonOutlineOutlinedIcon from '@mui/icons-material/PersonOutlineOutlined';

import { signOutNearWallet } from '../../../near-api';
import { Stack } from '@mui/system';
import { useDispatch, useSelector } from 'react-redux';
import { SET_PROFILE } from '../../redux/slices/user';

export default function AccountPopover() {
  const dispatch = useDispatch();
  const user = useSelector((state) => state.user);

  const [anchorEl, setAnchorEl] = useState(null);
  const [activeProfileDialog, setActiveProfileDialog] = useState(false);
  const open = Boolean(anchorEl);
  const handleClick = (event) => {
    if (window.walletConnection && window.walletConnection.isSignedIn()) {
      setAnchorEl(event.currentTarget);
    } else {
      navigate('/login', { replace: true });
    }
  };
  const handleClose = () => {
    setAnchorEl(null);
  };

  const handleOpenProfile = () => {
    setActiveProfileDialog(true);
  };

  console.log({ user });

  return (
    <Box>
      <IconButton onClick={handleClick}>
        <PersonOutlineOutlinedIcon />
      </IconButton>

      <Menu
        id="basic-menu"
        anchorEl={anchorEl}
        open={open}
        onClose={handleClose}
        MenuListProps={{
          'aria-labelledby': 'basic-button',
        }}
      >
        <MenuItem onClick={handleOpenProfile}>Profile</MenuItem>
        <MenuItem
          onClick={() => {
            signOutNearWallet();
            dispatch(SET_PROFILE({ isConnected: false }));
          }}
        >
          Logout
        </MenuItem>
      </Menu>

      <Dialog
        open={activeProfileDialog}
        onClose={() => setActiveProfileDialog(false)}
        aria-describedby="alert-dialog-slide-description"
      >
        <DialogTitle>Profile</DialogTitle>
        <DialogContent>
          <Stack>
            <Typography>
              Wallet address: <strong>{user.walletAddress}</strong>
            </Typography>
            <Typography>
              Balance: <strong>{(user.balance.total / 10e24).toFixed(2) + 'Ⓝ'}</strong>
            </Typography>
          </Stack>
        </DialogContent>
      </Dialog>
    </Box>
  );
}
