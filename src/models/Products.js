import { Schema, model } from "mongoose";

const ProductSchema = new Schema({
    name: String,
    slug: String,
    price: Number,
    is_sell: Boolean,
    quality: {
        certificate: String, 
        state: String, 
    },
    image: String,
    description: String
});

const ProductModel = model("products", ProductSchema);

export default ProductModel;