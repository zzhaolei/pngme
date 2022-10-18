# pngme
learning by https://picklenerd.github.io/pngme_book/introduction.html

# usage

- 隐藏数据
    ```shell
    cargo run -- encode WechatIMG49.png -c loVe -m "i love you ❤️" -o shadow.png
    ```

- 解码隐藏数据
    ```shell
    cargo run -- decode shadow.png -c loVe

    # output
    i love you ❤️
    ```

- 删除隐藏数据
    ```shell
    cargo run -- remove shadow.png -c loVe

    # output
    `loVe` message removed
    ```
