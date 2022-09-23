import { Router } from "express";
import OrderModel from "../models/Orders";
import ProductModel from "../models/Products";
import shortid from "shortid";
import blockchain from "../blockchain";
import configs from "../configs";
const router = Router();

router.get("/", (req, res) => {
    res.json({
        message: "Hello World"
    })
});

// API lay danh sach san pham
router.get("/products", async (req, res) => {
    try {
        let products = await ProductModel.find();

        res.json(products);
    } catch (error) {
        res.status(400).json({error: error.message});
    }
});

// API tao don hang
/**
 * ex body:
 * {
 *  product: "63171ef66ab4e10a3b8876ca",
 *  quantity: 2,
 *  accountId: "vbidev.testnet",
 *  customer: {
 *  ///
 * }
 * }
 */
router.post("/orders", async (req, res) => {
    try {
        let product = await ProductModel.findById(req.body.product);
        if (!product) {
            throw Error("Not found product id: " + req.body.product);
        }

        let totalAmount = product.price * req.body.quantity;
        let order = new OrderModel({
            ...req.body,
            orderCode: shortid.generate(),
            totalAmount,
            paymentStatus: "PENDING"
        });

        // console.log(order); 
        await order.save();

        if (order.is_sell == false)
        {
        let networkConfig = configs.getConfig("testnet");

        // Redirect user sang vi de thanh toan
        let signUrl = await blockchain.getSignUrl(
            order.accountId,
            "pay_order",
            {
                order_id: order.id,
                order_amount: blockchain.parseNearAmount(order.totalAmount)
            },
            order.totalAmount,
            30000000000000,
            networkConfig.paymentContract,
            "",
            "http://localhost:3000/api/payment-noti?orderId="+order.id,
            "testnet"
        )

        res.json({
            orderId: order.id,
            redirectUrl: signUrl
        });
    } else 
        res.json({
            orderId: order.id, 
            message: "Đơn hàng đã lưu trong database thanh cong"
        })

    } catch (error) {
        res.status(400).json({error: error.message});
    }
})


// API lay thong tin don hang
router.get("/orders/:orderId", async(req, res) => {
    try {
        let order = await OrderModel.findById(req.params.orderId).populate("product");
        if (!order) throw Error("Not found order id: " + req.params.orderId);
        res.json(order);
    } catch (error) {
        res.status(400).json({error: error.message});
    }
});

// kiem tra trang thai thanh toan va cap nhat database
router.get("/payment-noti", async (req, res) => {
    try {
        let order = await OrderModel.findById(req.query.orderId);
        if (!order || !req.query.orderId) throw Error("Not found order");

        if (order.paymentStatus == "PAID") {
            return res.json(order);
        }

        // Kiem tra trang thai thanh toan tren blockchain
        try {
            let networkConfig = configs.getConfig("testnet");
            let orderDetail = await blockchain.view(networkConfig.paymentContract, "get_order", {
                order_id: order.id
            });

            console.log("Order Detail: ", orderDetail);
            console.log("Amount: ", blockchain.parseNearAmount(order.totalAmount), orderDetail.received_amount.toLocaleString("fullwide", {useGrouping: false}));

            if (orderDetail.is_completed && blockchain.parseNearAmount(order.totalAmount) == orderDetail.received_amount.toLocaleString("fullwide", {useGrouping: false})) {
                order.paymentStatus = "PAID";
            } else {
                order.paymentStatus = "FAILED"
            }
        } catch (error) {
            order.paymentStatus = "FAIELD";
        }

        await order.save();

        res.json(order);

    } catch (error) {
        res.status(400).json({error: error.message});
    }
});


router.get("/test-get-order", async (req, res) => {
    try {
        let product = req.query.name_product;
        let sell = Boolean(req.query.is_sell); 

        console.log(product, sell);
        console.log(typeof(product), typeof(sell));
        let order = await blockchain.call(
            "manufacturing.uitdev.testnet",
            "get_product_order_way",
            {
                name_product: product,
                is_sell: sell
            },
            0,
            30000000000000
        )

        res.json(order);
    } catch (error) {
        res.status(400).json({error: error.message});
    }
})


router.get("/place_order", async (req, res) => {
    try {
        let command_id = req.query.command_id;
        let name_product = req.query.name_product;
        let sell = Boolean(req.query.is_sell);
        let amount_product = req.query.amount_product;
        let price_per_product = req.query.price_per_product;

        // let cert = req.query.quality.certificate;
        // let stage = req.query.quality.stage;

        console.log(command_id, typeof(command_id));

        let order = await blockchain.call(
            "manufacturing.uitdev.testnet",
            "add_command",
            {
                command_id: command_id,
                name_product: name_product,
                is_sell: sell, 
                amount_product: amount_product,
                price_per_product: price_per_product,
                quality: req.body.quality

                // command_id: "command1",
                // name_product: "asfasfasf",  
                // is_sell: sell, 
                // amount_product: "2",
                // price_per_product: "19000000000000000000000000",
                // quality: {
                //     certificate: ["asfasfasf"], 
                //     stage: ["asasfafa"]
                // }
            },
        )
        
        if (sell){
            var money = Number(req.query.amount_product * req.query.price_per_product);
            console.log(money);
            let order_id = req.query.command_id; 
            let networkConfig = configs.getConfig("testnet");

            // Redirect user sang vi de thanh toan
            let signUrl = await blockchain.getSignUrl(
                "hidang.testnet",
                "pay_order",
                {
                    order_id: "haidang",
                    order_amount: blockchain.parseNearAmount(money)
                },
                0,
                30000000000000,
                networkConfig.paymentContract,
                "",
                "http://localhost:3000/api/payment-noti?orderId="+"haidang",
                "testnet"
            )

            res.json({
                orderId: order.id,
                redirectUrl: signUrl,
                message: "Hay thanh toan"
            });
        } else 
            res.json({
                orderId: order.id, 
                message: "Đơn hàng đã lưu trong database thanh cong"
            })
        // res.json(order);
    } catch (error) {
        res.status(400).json({error: error.message});
    }
})



export default router;