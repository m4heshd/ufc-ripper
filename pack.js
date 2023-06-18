// Modules
const fs = require("fs-extra");
const path = require("path");
const clr = require('ansi-colors');
const project = require('./package.json');
const {exec} = require('pkg');
const ResEdit = require('resedit');
const zip = require('adm-zip');

let pkgDir = path.join(__dirname, 'package');
let outDir = path.join(pkgDir, 'win32');
let output = path.join(outDir, 'ufc-ripper.exe');
let outArchive = path.join(pkgDir, 'artifacts', 'ufc-ripper-win-x64.zip');
let target = 'node18-win-x64';

const platform = process.argv[2]?.trim() || 'win';

switch (platform) {
    case 'linux':
        outDir = path.join(pkgDir, 'linux');
        output = path.join(outDir, 'ufc-ripper');
        outArchive = path.join(pkgDir, 'artifacts', 'ufc-ripper-linux-x64.zip');
        target = 'node18-linux-x64';
    case 'win':
        runBuild();
        break;
    default:
        console.error(clr.redBright.bgBlack.bold('This platform is not supported.'));
}

function runBuild() {
    console.log(clr.cyanBright.bgBlack.bold('Starting build...\n'));

    exec([
        '.',
        // '--debug',
        '--targets', target,
        '--output', output
    ]).then(() => {
        try {
            fs.copySync(path.join(__dirname, 'config.json'), path.join(outDir, 'config.json'));
            if (platform === 'win') windowsPostBuild();
            createArchive();
        } catch (error) {
            console.error(clr.redBright.bgBlack.bold('Post-build process failed:'));
            console.error(error);
        }
    }).catch((error) => {
        console.error(clr.redBright.bgBlack.bold('Build process failed:'));
        console.error(error);
    });
}

function windowsPostBuild() {
    console.log(clr.cyanBright.bgBlack.bold('Updating icon resources...\n'));

    const exeBuffer = fs.readFileSync(output);
    const exe = ResEdit.NtExecutable.from(exeBuffer);
    const res = ResEdit.NtExecutableResource.from(exe);
    const iconFile = ResEdit.Data.IconFile.from(fs.readFileSync(path.join(__dirname, 'project-res', 'images', 'ufc-ripper-icon.ico')));

    ResEdit.Resource.IconGroupEntry.replaceIconsForResource(
        res.entries,
        1,
        1033,
        iconFile.icons.map(item => item.data)
    );

    console.log(clr.cyanBright.bgBlack.bold('Updating output metadata...\n'));

    const vi = ResEdit.Resource.VersionInfo.fromEntries(res.entries)[0];
    const version = project.version.split('.').map(v => Number(v));

    vi.setStringValues(
        {lang: 1033, codepage: 1200},
        {
            ProductName: 'UFC Ripper',
            FileDescription: project.description,
            CompanyName: 'm4heshd',
            LegalCopyright: `Copyright ${project.author}. ${project.license} license.`
        }
    );
    vi.removeStringValue({lang: 1033, codepage: 1200}, 'OriginalFilename');
    vi.removeStringValue({lang: 1033, codepage: 1200}, 'InternalName');
    vi.setFileVersion(...version, 0, 1033);
    vi.setProductVersion(...version, 0, 1033);
    vi.outputToResourceEntries(res.entries);

    console.log(clr.cyanBright.bgBlack.bold('Writing output file...\n'));

    res.outputResource(exe);
    fs.writeFileSync(output, Buffer.from(exe.generate()));
}

function createArchive() {
    console.log(clr.cyanBright.bgBlack.bold('Creating archive...\n'));

    const archive = new zip();

    archive.addLocalFolder(outDir);
    archive.writeZip(outArchive);
}
