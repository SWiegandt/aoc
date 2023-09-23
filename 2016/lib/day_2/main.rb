require_relative '../util'

class Day2
  def initialize
    @input = Util.read_input(2).split.map { |row| row.split(//) }
  end

  def main
    puts(problem_one)
    puts(problem_two)
  end

  def problem_one
    pos = [1, 1]
    code = ''

    @input.each do |row|
      row.each do |move|
        case move
        when 'R'
          pos[1] = (pos[1] + 1).clamp(0, 2)
        when 'L'
          pos[1] = (pos[1] - 1).clamp(0, 2)
        when 'U'
          pos[0] = (pos[0] - 1).clamp(0, 2)
        when 'D'
          pos[0] = (pos[0] + 1).clamp(0, 2)
        end
      end

      code += (pos[0] * 3 + pos[1] + 1).to_s
    end

    code
  end

  def problem_two
    keypad = [[nil, nil, 1, nil, nil], [nil, 2, 3, 4, nil], [5, 6, 7, 8, 9], [nil, 'A', 'B', 'C', nil],
              [nil, nil, 'D', nil, nil]]
    pos = [2, 0]
    code = ''

    @input.each do |row|
      row.each do |move|
        case move
        when 'R'
          next_pos = (pos[1] + 1).clamp(0, 4)
          pos[1] = next_pos unless keypad[pos[0]][next_pos].nil?
        when 'L'
          next_pos = (pos[1] - 1).clamp(0, 4)
          pos[1] = next_pos unless keypad[pos[0]][next_pos].nil?
        when 'U'
          next_pos = (pos[0] - 1).clamp(0, 4)
          pos[0] = next_pos unless keypad[next_pos][pos[1]].nil?
        when 'D'
          next_pos = (pos[0] + 1).clamp(0, 4)
          pos[0] = next_pos unless keypad[next_pos][pos[1]].nil?
        end
      end

      code += keypad[pos[0]][pos[1]].to_s
    end

    code
  end
end

Day2.new.main
