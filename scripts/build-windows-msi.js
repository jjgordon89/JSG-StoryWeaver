import { execSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// Get the directory name of the current module
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Ensure the wix directory exists
const wixDir = path.join(__dirname, '..', 'src-tauri', 'wix');
if (!fs.existsSync(wixDir)) {
  fs.mkdirSync(wixDir, { recursive: true });
  fs.mkdirSync(path.join(wixDir, 'fragments'), { recursive: true });
  console.log('Created WiX directories');
}

// Create a simple license file if it doesn't exist
const licensePath = path.join(__dirname, '..', 'LICENSE.rtf');
if (!fs.existsSync(licensePath)) {
  const licenseContent = `{\\rtf1\\ansi\\ansicpg1252\\deff0\\nouicompat\\deflang1033{\\fonttbl{\\f0\\fnil\\fcharset0 Calibri;}}
{\\*\\generator Riched20 10.0.19041}\\viewkind4\\uc1 
\\pard\\sa200\\sl276\\slmult1\\f0\\fs22\\lang9 StoryWeaver License Agreement\\par
Copyright (c) 2025 JSG StoryWeaver\\par
All rights reserved.\\par
By using this software, you agree to the terms and conditions of this license.\\par
}`;
  fs.writeFileSync(licensePath, licenseContent);
  console.log('Created license file');
}

// Create placeholder images for the installer if they don't exist
const bannerPath = path.join(wixDir, 'banner.png');
const dialogPath = path.join(wixDir, 'dialog.png');

if (!fs.existsSync(bannerPath)) {
  // Copy a placeholder image or create a simple one
  // For this example, we'll just copy an existing icon
  fs.copyFileSync(
    path.join(__dirname, '..', 'src-tauri', 'icons', '128x128.png'),
    bannerPath
  );
  console.log('Created banner image placeholder');
}

if (!fs.existsSync(dialogPath)) {
  // Copy a placeholder image or create a simple one
  fs.copyFileSync(
    path.join(__dirname, '..', 'src-tauri', 'icons', '128x128.png'),
    dialogPath
  );
  console.log('Created dialog image placeholder');
}

// Create a basic WiX template if it doesn't exist
const wixTemplatePath = path.join(wixDir, 'main.wxs');
if (!fs.existsSync(wixTemplatePath)) {
  const wixTemplateContent = `<?xml version="1.0" encoding="windows-1252"?>
<?if $(var.Platform) = x64 ?>
  <?define Win64 = "yes" ?>
  <?define PlatformProgramFilesFolder = "ProgramFiles64Folder" ?>
<?else ?>
  <?define Win64 = "no" ?>
  <?define PlatformProgramFilesFolder = "ProgramFilesFolder" ?>
<?endif ?>

<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">
  <Product
    Id="*"
    Name="StoryWeaver"
    UpgradeCode="5D2D1DF6-7D25-4D14-BB0A-1F73166F9C5A"
    Language="1033"
    Manufacturer="JSG StoryWeaver"
    Version="$(var.Version)">

    <Package
      Id="*"
      Keywords="Installer"
      Description="StoryWeaver Installer"
      Comments="StoryWeaver is an AI-powered writing assistant"
      Manufacturer="JSG StoryWeaver"
      InstallerVersion="200"
      Languages="1033"
      Compressed="yes"
      SummaryCodepage="1252"
      InstallScope="perMachine" />

    <MajorUpgrade
      DowngradeErrorMessage="A newer version of [ProductName] is already installed."
      AllowSameVersionUpgrades="yes" />

    <MediaTemplate EmbedCab="yes" />

    <Feature Id="ProductFeature" Title="StoryWeaver" Level="1">
      <ComponentGroupRef Id="ProductComponents" />
      <ComponentRef Id="ApplicationShortcut" />
      <ComponentRef Id="ApplicationShortcutDesktop" />
    </Feature>

    <Icon Id="ProductIcon" SourceFile="$(var.ProjectDir)/icons/icon.ico" />
    <Property Id="ARPPRODUCTICON" Value="ProductIcon" />
    <Property Id="ARPURLINFOABOUT" Value="https://github.com/jjgordon89/JSG-StoryWeaver" />
    <Property Id="ARPNOREPAIR" Value="yes" Secure="yes" />
    <SetProperty Id="ARPNOMODIFY" Value="1" After="InstallValidate" Sequence="execute" />

    <UI>
      <UIRef Id="WixUI_FeatureTree" />
      <Publish Dialog="ExitDialog"
        Control="Finish"
        Event="DoAction"
        Value="LaunchApplication">WIXUI_EXITDIALOGOPTIONALCHECKBOX = 1 and NOT Installed</Publish>
    </UI>

    <WixVariable Id="WixUILicenseRtf" Value="$(var.ProjectDir)/../LICENSE.rtf" />
    <WixVariable Id="WixUIBannerBmp" Value="$(var.ProjectDir)/wix/banner.png" />
    <WixVariable Id="WixUIDialogBmp" Value="$(var.ProjectDir)/wix/dialog.png" />

    <Property Id="WIXUI_EXITDIALOGOPTIONALCHECKBOX" Value="1" />
    <Property Id="WIXUI_EXITDIALOGOPTIONALCHECKBOXTEXT" Value="Launch StoryWeaver" />
    <Property Id="WixShellExecTarget" Value="[#StoryWeaverExe]" />
    <CustomAction Id="LaunchApplication"
      BinaryKey="WixCA"
      DllEntry="WixShellExec"
      Impersonate="yes" />
  </Product>

  <Fragment>
    <Directory Id="TARGETDIR" Name="SourceDir">
      <Directory Id="$(var.PlatformProgramFilesFolder)">
        <Directory Id="INSTALLFOLDER" Name="StoryWeaver">
          <!-- Application files will be installed here -->
        </Directory>
      </Directory>
      <Directory Id="ProgramMenuFolder">
        <Directory Id="ApplicationProgramsFolder" Name="StoryWeaver" />
      </Directory>
      <Directory Id="DesktopFolder" Name="Desktop" />
    </Directory>
  </Fragment>

  <Fragment>
    <DirectoryRef Id="ApplicationProgramsFolder">
      <Component Id="ApplicationShortcut" Guid="*">
        <Shortcut Id="ApplicationStartMenuShortcut"
          Name="StoryWeaver"
          Description="AI-powered writing assistant"
          Target="[#StoryWeaverExe]"
          WorkingDirectory="INSTALLFOLDER" />
        <RemoveFolder Id="RemoveApplicationProgramsFolder" Directory="ApplicationProgramsFolder" On="uninstall" />
        <RegistryValue Root="HKCU" Key="Software\\JSG\\StoryWeaver" Name="installed" Type="integer" Value="1" KeyPath="yes" />
      </Component>
    </DirectoryRef>
    <DirectoryRef Id="DesktopFolder">
      <Component Id="ApplicationShortcutDesktop" Guid="*">
        <Shortcut Id="ApplicationDesktopShortcut"
          Name="StoryWeaver"
          Description="AI-powered writing assistant"
          Target="[#StoryWeaverExe]"
          WorkingDirectory="INSTALLFOLDER" />
        <RegistryValue Root="HKCU" Key="Software\\JSG\\StoryWeaver" Name="installed_desktop" Type="integer" Value="1" KeyPath="yes" />
      </Component>
    </DirectoryRef>
  </Fragment>

  <Fragment>
    <ComponentGroup Id="ProductComponents" Directory="INSTALLFOLDER">
      <!-- Contents will be generated by Tauri -->
    </ComponentGroup>
  </Fragment>
</Wix>`;
  fs.writeFileSync(wixTemplatePath, wixTemplateContent);
  console.log('Created WiX template file');
}

// Copy the Windows config to the main tauri.conf.json for building
console.log('Copying Windows configuration...');
const windowsConfig = JSON.parse(fs.readFileSync(path.join(__dirname, '..', 'src-tauri', 'tauri.windows.conf.json'), 'utf8'));
const mainConfigPath = path.join(__dirname, '..', 'src-tauri', 'tauri.conf.json');
const mainConfig = JSON.parse(fs.readFileSync(mainConfigPath, 'utf8'));

// Merge the Windows-specific configuration
mainConfig.tauri.bundle.targets = windowsConfig.tauri.bundle.targets;
mainConfig.tauri.bundle.windows = windowsConfig.tauri.bundle.windows;

// Write the updated config
fs.writeFileSync(mainConfigPath, JSON.stringify(mainConfig, null, 2));
console.log('Updated tauri.conf.json with Windows-specific settings');

// Run the build command
console.log('Building Windows MSI installer...');
try {
  execSync('npm run tauri build', { stdio: 'inherit' });
  console.log('Build completed successfully!');
} catch (error) {
  console.error('Build failed:', error);
  process.exit(1);
} finally {
  // Restore the original config
  const originalConfig = JSON.parse(fs.readFileSync(path.join(__dirname, '..', 'src-tauri', 'tauri.conf.json.bak'), 'utf8'));
  fs.writeFileSync(mainConfigPath, JSON.stringify(originalConfig, null, 2));
  console.log('Restored original tauri.conf.json');
}
