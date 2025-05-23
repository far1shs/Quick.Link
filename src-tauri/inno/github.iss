; 动态路径定义（适配 GitHub Actions 工作目录）
#define MyAppName "Quick Link"
#define MyAppVersion GetEnv('APP_VERSION')
#define MyAppPublisher "Far1sh"
#define MyAppURL "https://app.far1sh.icu/quick_link"
#define MyAppExeName "quicklink.exe"
#define Path GetEnv('GITHUB_WORKSPACE')
#define BuildPath GetEnv('GITHUB_WORKSPACE') + "\src-tauri\target\release"
#define IconsPath GetEnv('GITHUB_WORKSPACE') + "\src-tauri\icons"

[Setup]
AppId={{D5D8F352-3CA1-4EA7-BB54-813D09639196}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
UninstallDisplayIcon={app}\{#MyAppExeName}
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
PrivilegesRequired=admin
; 输出到工作流临时目录
OutputDir={#BuildPath}\bundle\inno
OutputBaseFilename=Quick Link Setup
SetupIconFile={#IconsPath}\icon.ico
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "chinese"; MessagesFile: "compiler:{#Path}\src-tauri\inno\ChineseSimplified.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
; 使用动态构建路径
Source: "{#BuildPath}\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Registry]
Root: HKCR; Subkey: "qlink"; ValueType: string; ValueData: "URL:Quick Link Protocol"; Flags: uninsdeletekey
Root: HKCR; Subkey: "qlink"; ValueType: string; ValueName: "URL Protocol"; ValueData: ""
Root: HKCR; Subkey: "qlink\shell\open\command"; ValueType: string; ValueData: """{app}\{#MyAppExeName}"" ""%1"""

[UninstallDelete]
Type: filesandordirs; Name: "{app}"
Type: filesandordirs; Name: "{userappdata}\Roaming\icu.far1sh.app.quick-link"

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent