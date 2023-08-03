# readme

通过 PowerShell Remoting 可以连接到 Windows 服务器来传输文件或执行命令

```shell
Enable-PSRemoting -Force
```

从本地计算机复制文件到远程服务器

```shell
{
  $Username = <usr>
  $Password = ConvertTo-SecureString <pwd> -AsPlainText -Force
  $Credential = New-Object System.Management.Automation.PSCredential($Username,$Password)
  Copy-Item -Path <from> -Destination <to> -ToSession (New-PSSession -ComputerName <host> -Credential $Credential)
}
```