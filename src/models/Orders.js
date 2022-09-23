import { Schema, model, SchemaTypes } from "mongoose";

// Populate 
const OrderSchema = new Schema({
    commandID: String,
    product: {
        type: SchemaTypes.ObjectId,
        ref: "products"
    },
    is_sell: String, 
    quantity: Number,
    totalAmount: Number,
    accountId: String,
    paymentStatus: String, // PENDING, PAID, FAILED
    customer: {
        name: String,
        email: String,
        phone: String,
        address: String
    }
});

const OrderModel = model("orders", OrderSchema);
export default OrderModel;