
pub fn main() {

}

#[no_mangle]
pub extern fn show_emoji() -> *const u8 {
    let data = r#"<style>
            .wrapper {
                position: absolute;
                top: 50%;
                left: 50%;
                transform: translate(-50%, -50%);
                width: 100%;
                text-align: center;
            }

            .emoji {
                width: 150px;
                height: 150px;
                border-radius: 50%;
                background: yellow;
                display: inline-block;
                position: relative;
                margin: 20px 20px 0 20px;
            }

            .emoji .eyes {
                position: absolute;
                left: 50%;
                width: 60px;
                top: 80px;
            }

            .emoji .eyes::before,
            .emoji .eyes::after {
                content: '';
                background: black;
                width: 15px;
                height: 15px;
                border-radius: 50%;
            }

            .emoji .eyes::before {
                float: left;
            }

            .emoji .eyes::after {
                float: right;
                left: 100px;
                top: 50px;
            }

            .emoji.happy {
                transform: rotate(10deg);
                background: #47cf73;
            }

            .emoji.happy .mouth,
            .emoji.happy .eyes {
                animation: happy 1s ease-in-out infinite;
            }

            .emoji.happy .eyes::before,
            .emoji.happy .eyes::after {
                height: 15px;
                animation: blinkeyes 2s ease-in-out infinite;
            }

            .emoji.happy .mouth {
                position: absolute;
                left: 50%;
                top: 70%;
                transform: translate(-50%, -50%);
                background: black;
                width: 30px;
                height: 15px;
                border-bottom-left-radius: 30px;
                border-bottom-right-radius: 30px;
                border-top-left-radius: 10px;
                border-top-right-radius: 10px;
            }

            .emoji.happy .mouth::after {
                content: '';
                position: absolute;
                background: red;
                width: 10px;
                height: 5px;
                bottom: 2px;
                left: 50%;
                transform: translateX(-50%);
                border-radius: 50%;
            }
            @keyframes happy {
                0% {
                    transform: translate(-50%, -50%);
                }

                60% {
                    transform: translate(-50%, -100%);
                }

                100% {
                    transform: translate(-50%, -50%);
                }
            }

            @keyframes blinkeyes {
                0% {
                    transform: scaleY(1);
                }

                97% {
                    transform: scaleY(1);
                }

                100% {
                    transform: scaleY(0);
                }
            }
        </style>
        <div class="wrapper">
	        <span class="emoji happy">
		        <span class="eyes"></span>
		        <span class="mouth"></span>
	        </span>
        </div>"#;
    return data.as_ptr();
}