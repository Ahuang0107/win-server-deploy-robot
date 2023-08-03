# readme

通过 PowerShell Remoting 可以连接到 Windows 服务器来传输文件或执行命令

1. 首先需要开启 PowerShell Remoting

   ```shell
   Enable-PSRemoting -Force
   ```

2. 然后需要允许本机连接 remote 服务器

   ```shell
   Set-Item WSMan:localhost\client\trustedhosts -value * -Force
   ```

3. 创建与 remote 服务器的连接

    ```shell
    {
      $Username = <usr>
      $Password = ConvertTo-SecureString <pwd> -AsPlainText -Force
      $Credential = New-Object System.Management.Automation.PSCredential($Username,$Password)
      $Session = New-PSSession -ComputerName <host> -Credential $Credential
    }
    ```

4. 执行相关操作

    ```shell
    {
      ### 新建文件夹
      New-Item -ItemType Directory -Path <folder> -Force -ToSession $Session
      ### 移动文件
      Move-Item -Path <from> -Destination <to> -ToSession $Session
      ### 从本地复制文件到 remote 服务器
      Copy-Item -Path <from> -Destination <to> -ToSession $Session
      ### 在 remote 服务器上执行民命令
      Invoke-Command -ToSession $Session -ScriptBlock { <commands> }
    }
    ```