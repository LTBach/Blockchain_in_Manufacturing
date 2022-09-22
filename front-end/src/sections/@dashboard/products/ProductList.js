import PropTypes from 'prop-types';
// material
import { Grid } from '@mui/material';
import BlogPostCard from '../blog/BlogPostCard';

// ----------------------------------------------------------------------

ProductList.propTypes = {
  products: PropTypes.array.isRequired,
};

export default function ProductList({ products, ...other }) {
  return (
    <Grid container spacing={3} {...other}>
      {products.map((product, index) => (
        <BlogPostCard key={product.id} post={product} index={index} />
      ))}
    </Grid>
  );
}
