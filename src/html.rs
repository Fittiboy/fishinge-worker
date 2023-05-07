pub const SE_TOKEN_FORM: &str = r#"
    <!DOCTYPE html>
    <html>
    <head>
      <title>Pond Opener</title>
      <style>
        body {
          background-color: #444654;
          font-family: Arial, sans-serif;
          font-size: 16px;
          color: #ffffff;
          margin: 0;
          padding: 0;
        }

        .header {
          text-align: center;
          margin: 50px auto;
          max-width: 700px;
        }

        .header p {
          font-size: 18px;
          margin: 0;
        }

        form {
          margin: 50px auto;
          max-width: 600px;
          display: flex;
          align-items: center;
        }

        label {
          color: #ffffff;
          margin-right: 10px;
        }

        input[type="text"] {
          flex: 1;
          padding: 10px;
          font-size: 16px;
          border-radius: 3px;
          border: 1px solid #cccccc;
        }

        input[type="submit"] {
          padding: 10px 20px;
          font-size: 16px;
          background-color: #4e8cff;
          color: #ffffff;
          border: none;
          border-radius: 2px;
          cursor: pointer;
        }

        a {
          color: #4e8cff;
        }

        input[type="submit"]:hover {
          background-color: #2d6ec4;
        }
      </style>
    </head>
    <body>
      <div class="header">
        <p>
          <h2>To update your chat bot command, we will need your StreamElements
          token!</h2>
          To get this token, go
          <a href="https://streamelements.com/dashboard/account/channels">
            here</a>,
          click "Show secrets" towards the top right,
          and copy the very long string that shows up under "JWT Token."
          It should span multiple lines. Keep this token completely secret!
        </p>
      </div>
      <form action="https://fishinge.fitti.io/se_token" method="POST">
        <label for="seToken">SE token:</label>
        <input type="text" id="seToken" name="seToken" required>
        <input type="submit" value="Submit">
      </form>
    </body>
    </html>
"#;
