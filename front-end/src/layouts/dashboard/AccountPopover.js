import { useRef, useState } from 'react';
// @mui
import { alpha } from '@mui/material/styles';
import { IconButton } from '@mui/material';
// components
import PersonOutlineOutlinedIcon from '@mui/icons-material/PersonOutlineOutlined';

import { Link as RouterLink } from 'react-router-dom';

// ----------------------------------------------------------------------

// ----------------------------------------------------------------------

export default function AccountPopover() {
  return (
    <RouterLink to="/login">
      <IconButton>
        <PersonOutlineOutlinedIcon />
      </IconButton>
    </RouterLink>
  );
}
