import child_process from 'child_process';
import fs from 'fs';

function formatTemplate(str, args) {
  for (const arg in args) {
    str = str.replace(`\${${arg}}`, args[arg])
  }
  return str
}

function main() {
  const now = new Date()
  const date = `${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`

  const results = child_process.execSync(`cargo bench`).toString('utf-8')
  console.log(results)
  const resultsPath = `benchmark_results/${date}.txt`
  fs.writeFileSync(resultsPath, results)

  const template = fs.readFileSync('README.md.template', 'utf-8')
  const DO_NOT_EDIT = '<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->\n<!-- edit README.md.template instead -->\n\n'
  fs.writeFileSync(
    'README.md',
    DO_NOT_EDIT + formatTemplate(template, { date, results: format(results) })
  )
}

function parseUnit(unit) {
  switch (unit) {
    case 'ns':
      return 1;
    case 'µs':
      return 1000;
    case 'ms':
      return 1000000;
  }
}

function parseTime(time) {
  return Number(time.substr(0, time.length - 3)) * parseUnit(time.substr(time.length - 2, 2))
}

function getMinValue(dataset, bench) {
  let min = null
  for (let framework in dataset) {
    if (dataset[framework][bench] !== undefined) {
      let benchResults = dataset[framework][bench]
      if (benchResults.value != null && (min == null || benchResults.value < min)) {
        min = benchResults.value
      }

      for (let variant in benchResults.variants) {
        let variantResults = benchResults.variants[variant]
        if (variantResults.value != null && (min == null || variantResults.value < min)) {
          min = variantResults.value
        }
      }
    }
  }
  return min
}

function buildTables(results, dataset, columns, footnote) {
  let header = '| Format / Lib |'
  for (let column of columns) {
    header += ` ${column[0].toUpperCase() + column.substr(1)} |`
  }
  header += '\n|---|'
  for (let column of columns) {
    header += '--:|'
  }

  let dataTable = ''
  for (let framework in results[dataset]) {
    let hadResult = false
    let row = `| ${framework} |`
    for (let column of columns) {
      let output = results[dataset][framework][column]
      if (output === undefined) {
        row += ` ${footnote} |`
      } else {
        if (output.display != null) {
          row += ` ${output.display}`
          hadResult = true
        }
        for (let variant in output.variants) {
          row += ` <span title="${variant}">*${output.variants[variant].display}\\**</span>`
          hadResult = true
        }
        row += ' |'
      }
    }
    if (hadResult) {
      if (dataTable != '') {
        dataTable += '\n'
      }
      dataTable += `${row}`
    }
  }

  let comparisonTable = ''
  for (let framework in results[dataset]) {
    let hadResult = false
    let row = `| ${framework} |`
    for (let column of columns) {
      let min = getMinValue(results[dataset], column)
      let output = results[dataset][framework][column]
      if (output === undefined) {
        row += ` ${footnote} |`
      } else {
        if (output.value != null) {
          row += ` ${(min / output.value * 100).toFixed(2)}%`
          hadResult = true
        }
        for (let variant in output.variants) {
          row += ` <span title="${variant}">*${(min / output.variants[variant].value * 100).toFixed(2)}%\\**</span>`
          hadResult = true
        }
        row += ' |'
      }
    }
    if (hadResult) {
      if (comparisonTable != '') {
        comparisonTable += '\n'
      }
      comparisonTable += `${row}`
    }
  }

  return {
    header,
    data: dataTable,
    comparison: comparisonTable
  }
}

const DATASET_DESCRIPTIONS = {
  guilds: '24-ish [guilds](https://github.com/twilight-rs/twilight/blob/bb8f8f41251e99f4475f5a6757e5a0f84205a83c/twilight-model/src/guild/mod.rs#L78) (huge type, contains everything but messages)',
  messages: '500-ish [messages](https://github.com/twilight-rs/twilight/blob/bb8f8f41251e99f4475f5a6757e5a0f84205a83c/twilight-model/src/channel/message/mod.rs#L52) (without components)',
  channels: '750-ish [channels](https://github.com/twilight-rs/twilight/blob/bb8f8f41251e99f4475f5a6757e5a0f84205a83c/twilight-model/src/channel/mod.rs#L57)',
  members: '1000-ish [members](https://github.com/twilight-rs/twilight/blob/bb8f8f41251e99f4475f5a6757e5a0f84205a83c/twilight-model/src/guild/member.rs#L16)',
  roles: '2500-ish [roles](https://github.com/twilight-rs/twilight/blob/bb8f8f41251e99f4475f5a6757e5a0f84205a83c/twilight-model/src/guild/role.rs#L11)'
}

function format(input) {
  let bench_times_re = /^([a-z_\-]+)\/([a-z_\-]+)\/([a-z\-]+)(?: \(([a-z \-+]*)\))?\W+time:   \[\d+\.\d+ [µnm]s (\d+\.\d+ [µnm]s).*$/gm
  let bench_sizes_re = /^([a-z_\-]+)\/([a-z_\-]+)\/(size|zlib|zstd) (\d+)$/gm

  let results = {}
  for (let match of input.matchAll(bench_times_re)) {
    let dataset = match[1]
    let framework = match[2]
    let bench = match[3]
    let variant = match[4]
    let time = match[5]

    if (results[dataset] === undefined) {
      results[dataset] = {}
    }

    if (results[dataset][framework] === undefined) {
      results[dataset][framework] = {}
    }

    if (results[dataset][framework][bench] === undefined) {
      results[dataset][framework][bench] = {
        display: null,
        value: null,
        variants: {}
      }
    }

    let benchResults = results[dataset][framework][bench]

    if (variant == null) {
      benchResults.display = time
      benchResults.value = parseTime(time)
    } else {
      benchResults.variants[variant] = {
        display: time,
        value: parseTime(time)
      }
    }
  }

  for (let match of input.matchAll(bench_sizes_re)) {
    let dataset = match[1]
    let framework = match[2]
    let bench = match[3]
    let size = match[4]

    results[dataset][framework][bench] = {
      display: Number(size),
      value: Number(size)
    }
  }

  let tables = ''
  for (let dataset in results) {
    let frameworks = results[dataset]

    let serdeTables = buildTables(results, dataset, ['serialize', 'deserialize', 'size', 'zlib', 'zstd'], '†')

    tables += `\
## \`${dataset}\`

${DATASET_DESCRIPTIONS[dataset] || 'Missing dataset description'}

### Raw Data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

${serdeTables.header}
${serdeTables.data}

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

${serdeTables.header}
${serdeTables.comparison}

`
  }

  return tables
}

main()
